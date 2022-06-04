use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::function::Demand;
use crate::economy::function::FunctionAbstract;
use crate::economy::function::Supply;
use crate::economy::geography::CityId;
use crate::economy::geography::Geography;
use dashmap::DashMap;
use ordered_float::NotNan;
use rayon::prelude::*;
use std::collections::BTreeMap;

use super::types::Price;
use super::types::Volume;

#[derive(Copy, Clone, Debug)]
pub enum MarketState {
    Undefined,
    UnderSupply,
    OverSupply,
    Equilibrium(Price, Volume, Volume),
}

#[derive(Clone, Debug)]
pub struct CityData {
    demand: Demand,
    supply: Supply,
    state: MarketState,
}

impl CityData {
    fn new() -> CityData {
        CityData {
            demand: Demand::zero(),
            supply: Supply::zero(),
            state: MarketState::Undefined,
        }
    }

    pub fn demand(&self) -> &Demand {
        &self.demand
    }

    fn add_demand(&mut self, demand: &Demand) {
        self.demand.add_function(demand);
    }

    fn substract_demand(&mut self, demand: &Demand) {
        self.demand.substract_function(demand);
    }

    pub fn supply(&self) -> &Supply {
        &self.supply
    }

    fn add_supply(&mut self, supply: &Supply) {
        self.supply.add_function(supply);
    }

    fn substract_supply(&mut self, supply: &Supply) {
        self.supply.substract_function(supply);
    }

    pub fn state(&self) -> &MarketState {
        &self.state
    }

    fn set_state(&mut self, state: MarketState) {
        self.state = state;
    }

    pub fn price(&self) -> Option<Price> {
        if let MarketState::Equilibrium(price, _, _) = self.state {
            Some(price)
        } else {
            None
        }
    }

    pub fn demand_volume(&self) -> Option<Volume> {
        if let MarketState::Equilibrium(_, volume, _) = self.state {
            Some(volume)
        } else {
            None
        }
    }

    pub fn supply_volume(&self) -> Option<Volume> {
        if let MarketState::Equilibrium(_, _, volume) = self.state {
            Some(volume)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Market {
    geography: Geography,
    cities: DashMap<CityId, CityData>,
}

impl Market {
    pub fn new(geography: Geography, prices: BTreeMap<CityId, Price>) -> Market {
        let cities: DashMap<CityId, CityData> = geography
            .cities()
            .into_iter()
            .map(|x| {
                (x.id(), {
                    let mut data = CityData::new();
                    prices
                        .get(&x.id())
                        .map(|x| MarketState::Equilibrium(*x, Volume::zero(), Volume::zero()))
                        .into_iter()
                        .for_each(|x| data.set_state(x));
                    data
                })
            })
            .collect();
        Market { geography, cities }
    }

    pub fn geography(&self) -> &Geography {
        &self.geography
    }

    pub fn add_producer(&mut self, prod: &Producer) {
        self.cities
            .get_mut(&prod.city())
            .unwrap()
            .add_supply(prod.supply())
    }

    pub fn remove_producer(&mut self, prod: &Producer) {
        self.cities
            .get_mut(&prod.city())
            .unwrap()
            .substract_supply(prod.supply())
    }

    pub fn add_consumer(&mut self, cons: &Consumer) {
        self.cities
            .get_mut(&cons.city())
            .unwrap()
            .add_demand(cons.demand())
    }

    pub fn remove_consumer(&mut self, cons: &Consumer) {
        self.cities
            .get_mut(&cons.city())
            .unwrap()
            .substract_demand(cons.demand())
    }

    pub fn prices(&self) -> BTreeMap<CityId, Option<Price>> {
        self.cities.iter().map(|x| (*x.key(), x.price())).collect()
    }

    pub fn demand_volumes(&self) -> BTreeMap<CityId, Option<Volume>> {
        self.cities
            .iter()
            .map(|x| (*x.key(), x.demand_volume()))
            .collect()
    }

    pub fn supply_volumes(&self) -> BTreeMap<CityId, Option<Volume>> {
        self.cities
            .iter()
            .map(|x| (*x.key(), x.supply_volume()))
            .collect()
    }

    fn calculate_groups_dfs(
        &self,
        pos: CityId,
        group_id: CityId,
        group_diff: Price,
        groups: &mut BTreeMap<CityId, (CityId, Price)>,
    ) {
        if groups.contains_key(&pos) {
            return;
        }
        groups.insert(pos, (group_id, group_diff));

        let connections = self.geography.connections();
        for conn in connections[pos] {
            let id_from = conn.id_from();
            let id_to = conn.id_to();
            let cost = conn.cost();

            let (price_from, price_to) = match (
                self.cities.get(&id_from).unwrap().state,
                self.cities.get(&id_to).unwrap().state,
            ) {
                (
                    MarketState::Equilibrium(price_from, _, _),
                    MarketState::Equilibrium(price_to, _, _),
                ) => (price_from, price_to),
                (MarketState::OverSupply, MarketState::Equilibrium(price_to, _, _)) => {
                    (Price::min(), price_to)
                }
                (MarketState::UnderSupply, MarketState::Equilibrium(price_to, _, _)) => {
                    (Price::max(), price_to)
                }
                (MarketState::Equilibrium(price_from, _, _), MarketState::OverSupply) => {
                    (price_from, Price::min())
                }
                (MarketState::Equilibrium(price_from, _, _), MarketState::UnderSupply) => {
                    (price_from, Price::max())
                }
                (MarketState::UnderSupply, MarketState::OverSupply) => (Price::max(), Price::min()),
                (MarketState::OverSupply, MarketState::UnderSupply) => (Price::min(), Price::max()),
                _ => {
                    // Initiates identical values so that they will be only connected when transport between them is free.
                    (Price::new(0.), Price::new(0.))
                }
            };

            if price_from - price_to >= cost || price_to - price_from >= cost {
                self.calculate_groups_dfs(
                    id_to,
                    group_id,
                    group_diff + cost * (if price_to > price_from { 1. } else { -1. }),
                    groups,
                )
            }
        }
    }

    fn calculate_groups(&self) -> BTreeMap<CityId, Vec<(CityId, Price)>> {
        // Map id -> (group_id, price_compared_to_groups_base).
        let mut groups: BTreeMap<CityId, (CityId, Price)> = BTreeMap::new();
        for entry in &self.cities {
            let i = entry.key();
            self.calculate_groups_dfs(*i, *i, Price::new(0.), &mut groups);
        }

        // Map group_id -> [(id, price_compared_to_groups_base)].
        let mut group_lists: BTreeMap<CityId, Vec<(CityId, Price)>> =
            self.cities.iter().map(|x| (*x.key(), vec![])).collect();
        for city in groups {
            group_lists
                .get_mut(&city.1 .0)
                .unwrap()
                .push((city.0, city.1 .1));
        }
        group_lists
    }

    pub fn update_prices(&mut self) {
        let group_lists = self.calculate_groups();

        group_lists.par_iter().for_each(|group| {
            let (demand, supply) = group
                .1
                .par_iter()
                .map(|(city_id, price_diff)| {
                    let city = &self.cities.get(city_id).unwrap();
                    let mut city_demand = city.demand().clone();
                    let mut city_supply = city.supply().clone();
                    city_demand.shift_left(*price_diff);
                    city_supply.shift_left(*price_diff);
                    (city_demand, city_supply)
                })
                .reduce(
                    || (Demand::zero(), Supply::zero()),
                    |(mut demand_1, mut supply_1), (demand_2, supply_2)| {
                        demand_1.add_function(&demand_2);
                        supply_1.add_function(&supply_2);
                        (demand_1, supply_1)
                    },
                );

            let state_global = demand.intersect(&supply);

            for (city_id, price_diff) in group.1 {
                let mut city_state = self.cities.get_mut(city_id).unwrap();
                let new_state = match state_global {
                    MarketState::Equilibrium(price, _, _) => {
                        let price_local = price + *price_diff;
                        let demand = city_state.demand().value(price_local);
                        let supply = city_state.supply().value(price_local);
                        MarketState::Equilibrium(price_local, demand, supply)
                    }
                    state => state,
                };
                city_state.set_state(new_state);
            }
        });
    }

    pub fn simulate(&mut self, tours: u32) {
        for _ in 0..tours {
            self.update_prices();
        }
    }

    pub fn reset_prices(&mut self) {
        self.cities
            .iter_mut()
            .for_each(|mut city| city.set_state(MarketState::Undefined));
    }
}

#[cfg(test)]
pub mod tests {
    use crate::economy::entity::Consumer;
    use crate::economy::entity::Producer;
    use crate::economy::function::Demand;
    use crate::economy::function::Function;
    use crate::economy::function::Supply;
    use crate::economy::geography::City;
    use crate::economy::geography::CityId;
    use crate::economy::geography::Connection;
    use crate::economy::geography::Geography;
    use crate::economy::market::CityData;
    use crate::economy::market::Market;
    use crate::economy::market::MarketState;
    use crate::economy::types::InnerValue;
    use crate::economy::types::Price;
    use crate::economy::types::Volume;
    use crate::util::testing::make_function;
    use crate::util::testing::test_eq_arg;
    use crate::util::testing::test_eq_value;

    use dashmap::DashMap;
    use ordered_float::NotNan;
    use std::collections::BTreeMap;

    fn generateCities(
        geography: &Geography,
        prices_vec: Vec<(CityId, InnerValue)>,
    ) -> DashMap<CityId, CityData> {
        let prices: BTreeMap<CityId, InnerValue> = prices_vec.into_iter().collect();
        geography
            .cities
            .iter()
            .map(|x| {
                let demand = Demand::zero();
                let supply = Supply::zero();
                let state = MarketState::Equilibrium(
                    Price::new(prices[&x.0]),
                    Volume::zero(),
                    Volume::zero(),
                );
                (
                    *x.0,
                    CityData {
                        demand,
                        supply,
                        state,
                    },
                )
            })
            .collect()
    }

    #[cfg(test)]
    pub mod groups {
        use super::*;

        fn test_groups(market: &Market, groups: &BTreeMap<CityId, Vec<(CityId, Price)>>) {
            let mut id_to_group: BTreeMap<CityId, CityId> = BTreeMap::new();
            let prices: BTreeMap<CityId, Price> = market
                .prices()
                .iter()
                .map(|x| (*x.0, x.1.unwrap()))
                .collect();

            for (base, group) in groups {
                for (id, diff) in group {
                    id_to_group.insert(*id, *base);
                }
            }

            for vec in market.geography.connections() {
                for conn in vec {
                    let from = &conn.id_from();
                    let to = &conn.id_to();
                    if id_to_group[from] != id_to_group[to] {
                        assert!(
                            prices[from] - prices[to] < conn.cost()
                                && prices[to] - prices[from] < conn.cost()
                        )
                    }
                }
            }
        }

        #[test]
        pub fn two_nodes_two_groups() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, Price::new(100.)));

            let cities = generateCities(&geography, vec![(0, 5.), (1, 7.)]);

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 2);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn two_nodes_one_group_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, Price::new(5.)));

            let cities = generateCities(&geography, vec![(0, 5.), (1, 25.)]);

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 1);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn two_nodes_one_group_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, Price::new(5.)));

            let cities = generateCities(&geography, vec![(0, 0.), (1, 20.)]);

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 1);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn three_nodes_two_groups() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_city(City::new(2, String::new()));

            geography.add_connection(Connection::new(0, 1, Price::new(5.)));
            geography.add_connection(Connection::new(1, 2, Price::new(100.)));
            geography.add_connection(Connection::new(0, 2, Price::new(100.)));

            let cities = generateCities(&geography, vec![(0, 5.), (1, 25.), (2, 30.)]);

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 2);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn three_nodes_one_group() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_city(City::new(2, String::new()));

            geography.add_connection(Connection::new(0, 1, Price::new(5.)));
            geography.add_connection(Connection::new(1, 2, Price::new(5.)));
            geography.add_connection(Connection::new(0, 2, Price::new(100.)));

            let cities = generateCities(&geography, vec![(0, 5.), (1, 25.), (2, 45.)]);

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 1);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn five_nodes_two_groups() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_city(City::new(2, String::new()));
            geography.add_city(City::new(3, String::new()));
            geography.add_city(City::new(4, String::new()));

            geography.add_connection(Connection::new(0, 1, Price::new(5.)));
            geography.add_connection(Connection::new(1, 2, Price::new(5.)));
            geography.add_connection(Connection::new(0, 2, Price::new(5.)));
            geography.add_connection(Connection::new(1, 3, Price::new(100.)));
            geography.add_connection(Connection::new(0, 4, Price::new(100.)));
            geography.add_connection(Connection::new(2, 3, Price::new(100.)));
            geography.add_connection(Connection::new(3, 4, Price::new(5.)));

            let cities = generateCities(
                &geography,
                vec![(0, 5.), (1, 25.), (2, 45.), (3, 20.), (4, 10.)],
            );

            let market = Market { geography, cities };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 2);
            test_groups(&market, &groups);
        }
    }

    #[cfg(test)]
    mod calculation {
        use crate::util::testing::{make_demand, make_supply};

        use super::*;

        #[test]
        fn single_node_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption = Consumer::new(0, make_demand(vec![(0., 4.), (4., 0.)]));
            let city_production = Producer::new(0, make_supply(vec![(0., 0.), (4., 4.)]));

            let mut market = Market::new(geography, BTreeMap::new());
            market.add_consumer(&city_consumption);
            market.add_producer(&city_production);

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.));

            market.update_prices();
            let prices = market.prices();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.));
        }

        #[test]
        fn single_node_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption = Consumer::new(0, make_demand(vec![(1., 5.), (5., 0.)]));
            let city_production =
                Producer::new(0, make_supply(vec![(0., 0.), (2., 1.), (4., 4.), (6., 6.)]));

            let mut market = Market::new(geography, BTreeMap::new());
            market.add_consumer(&city_consumption);
            market.add_producer(&city_production);

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(3.));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.5));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.5));

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(3.));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.5));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.5));
        }

        #[test]
        fn single_node_3() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption = Consumer::new(0, make_demand(vec![(3., 4.), (5., 1.)]));
            let city_production =
                Producer::new(0, make_supply(vec![(0., 1.), (2., 2.), (3., 6.), (5., 8.)]));

            let mut market = Market::new(geography, BTreeMap::new());
            market.add_consumer(&city_consumption);
            market.add_producer(&city_production);

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.5));
            test_eq_value(demands[&0].unwrap(), Volume::new(4.));
            test_eq_value(supplies[&0].unwrap(), Volume::new(4.));

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.5));
            test_eq_value(demands[&0].unwrap(), Volume::new(4.));
            test_eq_value(supplies[&0].unwrap(), Volume::new(4.));
        }

        #[test]
        fn two_nodes_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_connection(Connection::new(0, 1, Price::new(4.)));

            let city_0_consumption = Consumer::new(
                0,
                make_demand(vec![(0., 6.), (1., 5.), (2., 3.), (3., 2.), (4., 0.)]),
            );
            let city_0_production =
                Producer::new(0, make_supply(vec![(1., 0.), (2., 1.), (3., 3.), (5., 4.)]));
            let city_1_consumption = Consumer::new(
                1,
                make_demand(vec![(5., 9.), (7., 7.), (8., 4.), (9., 2.), (11., 1.)]),
            );
            let city_1_production = Producer::new(
                1,
                make_supply(vec![(6., 0.), (8., 2.), (9., 5.), (10., 6.)]),
            );

            let mut market = Market::new(geography, BTreeMap::new());
            market.add_consumer(&city_0_consumption);
            market.add_producer(&city_0_production);
            market.add_consumer(&city_1_consumption);
            market.add_producer(&city_1_production);

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.666666666));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.33333333));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.3333333));
            test_eq_arg(prices[&1].unwrap(), Price::new(8.4));
            test_eq_value(demands[&1].unwrap(), Volume::new(3.2));
            test_eq_value(supplies[&1].unwrap(), Volume::new(3.2));

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(3.769230769));
            test_eq_value(demands[&0].unwrap(), Volume::new(0.46153855));
            test_eq_value(supplies[&0].unwrap(), Volume::new(3.38461536));
            test_eq_arg(prices[&1].unwrap(), Price::new(7.769230769));
            test_eq_value(demands[&1].unwrap(), Volume::new(4.6923078));
            test_eq_value(supplies[&1].unwrap(), Volume::new(1.7692307));

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(3.769230769));
            test_eq_value(demands[&0].unwrap(), Volume::new(0.46153855));
            test_eq_value(supplies[&0].unwrap(), Volume::new(3.38461536));
            test_eq_arg(prices[&1].unwrap(), Price::new(7.769230769));
            test_eq_value(demands[&1].unwrap(), Volume::new(4.6923078));
            test_eq_value(supplies[&1].unwrap(), Volume::new(1.7692307));
        }

        #[test]
        fn two_nodes_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_connection(Connection::new(0, 1, Price::new(10.)));

            let city_0_consumption = Consumer::new(
                0,
                make_demand(vec![(0., 6.), (1., 5.), (2., 3.), (3., 2.), (4., 0.)]),
            );
            let city_0_production =
                Producer::new(0, make_supply(vec![(1., 0.), (2., 1.), (3., 3.), (5., 4.)]));
            let city_1_consumption = Consumer::new(
                1,
                make_demand(vec![(6., 0.), (8., 2.), (9., 5.), (10., 6.)]),
            );
            let city_1_production = Producer::new(
                1,
                make_supply(vec![(5., 9.), (7., 7.), (8., 4.), (9., 2.), (11., 1.)]),
            );

            let mut market_base = Market::new(geography, BTreeMap::new());
            market_base.add_consumer(&city_0_consumption);
            market_base.add_producer(&city_0_production);
            market_base.add_consumer(&city_1_consumption);
            market_base.add_producer(&city_1_production);
            let mut market = Market {
                geography: market_base.geography,
                cities: market_base.cities,
            };

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.666666666));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.33333333));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.3333333));
            test_eq_arg(prices[&1].unwrap(), Price::new(8.4));
            test_eq_value(demands[&1].unwrap(), Volume::new(3.2));
            test_eq_value(supplies[&1].unwrap(), Volume::new(3.2));

            market.update_prices();
            let prices = market.prices();
            let demands = market.demand_volumes();
            let supplies = market.supply_volumes();
            test_eq_arg(prices[&0].unwrap(), Price::new(2.666666666));
            test_eq_value(demands[&0].unwrap(), Volume::new(2.33333333));
            test_eq_value(supplies[&0].unwrap(), Volume::new(2.3333333));
            test_eq_arg(prices[&1].unwrap(), Price::new(8.4));
            test_eq_value(demands[&1].unwrap(), Volume::new(3.2));
            test_eq_value(supplies[&1].unwrap(), Volume::new(3.2));
        }

        #[test]
        fn three_node_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_city(City::new(2, "city 2".to_string()));
            geography.add_connection(Connection::new(0, 1, Price::new(2.)));
            geography.add_connection(Connection::new(1, 2, Price::new(1.)));

            let city_0_consumption =
                Consumer::new(0, make_demand(vec![(0., 8.), (1., 7.), (3., 3.), (5., 1.)]));
            let city_0_production =
                Producer::new(0, make_supply(vec![(0., 2.), (1., 3.), (3., 7.), (5., 8.)]));
            let city_1_consumption =
                Consumer::new(1, make_demand(vec![(3., 8.), (4., 6.), (5., 3.), (7., 2.)]));
            let city_1_production =
                Producer::new(1, make_supply(vec![(2., 1.), (4., 3.), (5., 5.), (6., 6.)]));
            let city_2_consumption =
                Consumer::new(2, make_demand(vec![(5., 6.), (6., 5.), (7., 3.), (9., 1.)]));
            let city_2_production = Producer::new(
                2,
                make_supply(vec![(3., 1.), (6., 3.), (8., 5.), (10., 6.)]),
            );

            let mut market = Market::new(geography, BTreeMap::new());
            market.add_consumer(&city_0_consumption);
            market.add_producer(&city_0_production);
            market.add_consumer(&city_1_consumption);
            market.add_producer(&city_1_production);
            market.add_consumer(&city_2_consumption);
            market.add_producer(&city_2_production);

            market.update_prices();
            let prices = market.prices();
            let price_0 = prices[&0].unwrap();
            let price_1 = prices[&1].unwrap();
            let price_2 = prices[&2].unwrap();
            test_eq_arg(price_0, Price::new(2.));
            test_eq_arg(price_1, Price::new(4.6));
            test_eq_arg(price_2, Price::new(6.666666666));

            market.update_prices();
            let prices = market.prices();
            let price_0 = prices[&0].unwrap();
            let price_1 = prices[&1].unwrap();
            let price_2 = prices[&2].unwrap();
            test_eq_arg(price_0, Price::new(2.6249999));
            test_eq_arg(price_1, Price::new(4.6249999));
            test_eq_arg(price_2, Price::new(5.6249999));

            market.update_prices();
            let prices = market.prices();
            let price_0 = prices[&0].unwrap();
            let price_1 = prices[&1].unwrap();
            let price_2 = prices[&2].unwrap();
            test_eq_arg(price_0, Price::new(2.6249999));
            test_eq_arg(price_1, Price::new(4.6249999));
            test_eq_arg(price_2, Price::new(5.6249999));
        }
    }
}

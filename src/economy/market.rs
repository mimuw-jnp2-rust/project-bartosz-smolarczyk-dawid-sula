use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::function::Function;
use crate::economy::geography::CityId;
use crate::economy::geography::Geography;
use crate::util::types::Value;
use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CityPrice {
    pub city: CityId,
    pub price: Value,
}

#[derive(Clone, Debug)]
pub struct CityData {
    demand: Function,
    supply: Function,
}

impl CityData {
    fn new() -> CityData {
        CityData {
            demand: Function::zero(),
            supply: Function::zero(),
        }
    }

    pub fn get_demand(&self) -> &Function {
        &self.demand
    }

    fn add_demand(&mut self, demand: &Function) {
        self.demand.add_function(demand);
    }

    fn substract_demand(&mut self, demand: &Function) {
        self.demand.substract_function(demand);
    }

    pub fn get_supply(&self) -> &Function {
        &self.supply
    }

    fn add_supply(&mut self, supply: &Function) {
        self.supply.add_function(supply);
    }

    fn substract_supply(&mut self, supply: &Function) {
        self.supply.substract_function(supply);
    }
}

#[derive(Debug)]
pub struct Market {
    geography: Geography,
    cities: BTreeMap<CityId, CityData>,

    /// Currect prices in cities. All values are 0 if they haven't
    /// been calculated yet.
    prices: BTreeMap<CityId, Value>,
}

impl Market {
    pub fn new(geography: Geography) -> Market {
        let cities: BTreeMap<CityId, CityData> = geography
            .get_cities()
            .into_iter()
            .map(|x| (x.get_id(), CityData::new()))
            .collect();
        let prices: BTreeMap<CityId, Value> = geography
            .get_cities()
            .into_iter()
            .map(|x| (x.get_id(), 0))
            .collect();
        Market {
            geography,
            cities,
            prices,
        }
    }

    pub fn get_geography(&self) -> &Geography {
        &self.geography
    }

    pub fn add_producer(&mut self, prod: &Producer) {
        self.cities
            .get_mut(&prod.get_city())
            .unwrap()
            .add_supply(prod.get_supply())
    }

    pub fn remove_producer(&mut self, prod: &Producer) {
        self.cities
            .get_mut(&prod.get_city())
            .unwrap()
            .substract_supply(prod.get_supply())
    }

    pub fn add_consumer(&mut self, cons: &Consumer) {
        self.cities
            .get_mut(&cons.get_city())
            .unwrap()
            .add_demand(cons.get_demand())
    }

    pub fn remove_consumer(&mut self, cons: &Consumer) {
        self.cities
            .get_mut(&cons.get_city())
            .unwrap()
            .substract_demand(cons.get_demand())
    }

    pub fn get_prices(&self) -> &BTreeMap<CityId, Value> {
        &self.prices
    }

    fn calculate_groups_dfs(
        &self,
        pos: CityId,
        group_id: CityId,
        group_diff: Value,
        groups: &mut BTreeMap<CityId, (CityId, Value)>,
    ) {
        if groups.contains_key(&pos) {
            return;
        }
        groups.insert(pos, (group_id, group_diff));

        let connections = self.geography.get_connections();
        for conn in connections[pos] {
            let id_from = conn.get_from_id();
            let id_to = conn.get_to_id();
            let cost = conn.get_cost();

            let price_from = self.prices[&id_from];
            let price_to = self.prices[&id_to];

            if (price_from - price_to).abs() >= cost {
                self.calculate_groups_dfs(
                    id_to,
                    group_id,
                    group_diff + cost * (price_to - price_from).signum(),
                    groups,
                )
            }
        }
    }

    fn calculate_groups(&self) -> BTreeMap<CityId, Vec<(CityId, Value)>> {
        // Map id -> (group_id, price_compared_to_groups_base).
        let mut groups: BTreeMap<CityId, (CityId, Value)> = BTreeMap::new();
        for i in self.cities.keys() {
            self.calculate_groups_dfs(*i, *i, 0, &mut groups);
        }

        // Map group_id -> [(id, price_compared_to_groups_base)].
        let mut group_lists: BTreeMap<CityId, Vec<(CityId, Value)>> =
            self.cities.keys().map(|x| (*x, vec![])).collect();
        for city in groups {
            group_lists
                .get_mut(&city.1 .0)
                .unwrap()
                .push((city.0, city.1 .1));
        }
        group_lists
    }

    pub fn change_price(&mut self, city_id: &CityId, price: &Value) {
        *self.prices.get_mut(city_id).unwrap() = *price;
    }

    pub fn update_prices(&mut self) {
        let group_lists = self.calculate_groups();

        for group in group_lists {
            let mut demand = Function::zero();
            let mut supply = Function::zero();

            for (city_id, price_diff) in &group.1 {
                let city = &self.cities[city_id];
                demand.add_function(city.get_demand().clone().shift(*price_diff));
                supply.add_function(city.get_supply().clone().shift(*price_diff));
            }

            let price = demand.intersect_with_supply(&supply);
            for (city_id, price_diff) in &group.1 {
                println! {"update_prices: {} {}", price, price_diff};
                *self.prices.get_mut(city_id).unwrap() = price + price_diff;
            }
        }
    }

    pub fn reset_prices(&mut self) {
        let prices: BTreeMap<CityId, Value> = self
            .geography
            .get_cities()
            .into_iter()
            .map(|x| (x.get_id(), 0))
            .collect();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::economy::entity::Consumer;
    use crate::economy::entity::Producer;
    use crate::economy::function::Function;
    use crate::economy::geography::City;
    use crate::economy::geography::CityId;
    use crate::economy::geography::Connection;
    use crate::economy::geography::Geography;
    use crate::economy::market::CityData;
    use crate::economy::market::Market;
    use crate::economy::market::Value;
    use std::collections::BTreeMap;

    #[cfg(test)]
    pub mod groups {
        use super::*;

        // Note that supply and demand do not affect this tests so they are
        // always initiated to Function::zero().

        fn generate_default_cities(geography: &Geography) -> BTreeMap<CityId, CityData> {
            let cities = geography.get_cities();
            BTreeMap::from_iter(cities.into_iter().map(|x| (x.get_id(), CityData::new())))
        }

        fn test_groups(market: &Market, groups: &BTreeMap<CityId, Vec<(CityId, Value)>>) {
            let mut id_to_group: BTreeMap<CityId, CityId> = BTreeMap::new();
            let prices = market.get_prices();

            for (base, group) in groups {
                for (id, diff) in group {
                    id_to_group.insert(*id, *base);
                }
            }

            for vec in market.geography.get_connections() {
                for conn in vec {
                    let from = &conn.get_from_id();
                    let to = &conn.get_to_id();
                    if id_to_group[from] != id_to_group[to] {
                        assert!((prices[from] - prices[to]).abs() < conn.get_cost())
                    }
                }
            }
        }

        #[test]
        pub fn two_nodes_two_groups() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, 100));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 5), (1, 7)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 2);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn two_nodes_one_group_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, 5));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 5), (1, 25)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 1);
            test_groups(&market, &groups);
        }

        #[test]
        pub fn two_nodes_one_group_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, String::new()));
            geography.add_city(City::new(1, String::new()));
            geography.add_connection(Connection::new(0, 1, 5));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 0), (1, Value::MAX)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
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

            geography.add_connection(Connection::new(0, 1, 5));
            geography.add_connection(Connection::new(1, 2, 100));
            geography.add_connection(Connection::new(0, 2, 100));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 5), (1, 25), (2, 30)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
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

            geography.add_connection(Connection::new(0, 1, 5));
            geography.add_connection(Connection::new(1, 2, 5));
            geography.add_connection(Connection::new(0, 2, 100));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 5), (1, 25), (2, 45)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
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

            geography.add_connection(Connection::new(0, 1, 5));
            geography.add_connection(Connection::new(1, 2, 5));
            geography.add_connection(Connection::new(0, 2, 5));
            geography.add_connection(Connection::new(1, 3, 100));
            geography.add_connection(Connection::new(0, 4, 100));
            geography.add_connection(Connection::new(2, 3, 100));
            geography.add_connection(Connection::new(3, 4, 5));

            let cities = generate_default_cities(&geography);
            let prices = BTreeMap::from([(0, 5), (1, 25), (2, 45), (3, 20), (4, 10)]);

            let market = Market {
                geography,
                cities,
                prices,
            };
            let groups = market.calculate_groups();

            assert_eq!(groups.iter().filter(|(_, v)| v.len() != 0).count(), 2);
            test_groups(&market, &groups);
        }
    }

    #[cfg(test)]
    mod blackbox {
        use super::*;

        #[test]
        fn single_node_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption = Consumer::new(0, Function::new(0, vec![4, 3, 2, 1, 0]));
            let city_production = Producer::new(0, Function::new(0, vec![0, 1, 2, 3, 4]));

            let mut market = Market::new(geography);
            market.add_consumer(&city_consumption);
            market.add_producer(&city_production);

            market.update_prices();
            let prices = market.get_prices();

            assert_eq!(prices[&0], 2);
        }

        #[test]
        fn single_node_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption = Consumer::new(0, Function::new(0, vec![8, 6, 5, 4, 3, 2, 1, 0]));
            let city_production = Producer::new(0, Function::new(0, vec![0, 1, 2, 2, 3, 3, 3]));

            let mut market = Market::new(geography);
            market.add_consumer(&city_consumption);
            market.add_producer(&city_production);

            market.update_prices();
            let prices = market.get_prices();

            assert_eq!(prices[&0], 4);
        }

        #[test]
        fn single_node_3() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city".to_string()));

            let city_consumption_1 = Consumer::new(0, Function::new(1, vec![10, 8, 5, 3, 2, 0]));
            let city_consumption_2 =
                Consumer::new(0, Function::new(0, vec![8, 6, 5, 4, 3, 2, 1, 0]));
            let city_production_1 = Producer::new(0, Function::new(0, vec![0, 1, 4, 7]));
            let city_production_2 = Producer::new(0, Function::new(1, vec![0, 1, 2, 2, 3, 3, 3]));

            let mut market = Market::new(geography);
            market.add_consumer(&city_consumption_1);
            market.add_consumer(&city_consumption_2);
            market.add_producer(&city_production_1);
            market.add_producer(&city_production_2);

            market.update_prices();
            let prices = market.get_prices();

            assert_eq!(prices[&0], 3);
        }

        #[test]
        fn two_nodes_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_connection(Connection::new(0, 1, 10));

            let city_0_consumption = Consumer::new(0, Function::new(0, vec![13, 13, 12, 11, 13]));
            let city_0_production = Producer::new(0, Function::new(0, vec![11, 11, 12, 13, 13]));
            let city_1_consumption = Consumer::new(1, Function::new(20, vec![3, 3, 2, 1, 1]));
            let city_1_production = Producer::new(1, Function::new(20, vec![1, 1, 2, 3, 3]));

            let mut market = Market::new(geography);
            market.add_consumer(&city_0_consumption);
            market.add_producer(&city_0_production);
            market.add_consumer(&city_1_consumption);
            market.add_producer(&city_1_production);

            market.update_prices();
            let prices = market.get_prices();

            assert_eq!(prices[&0], 2);
            assert_eq!(prices[&1], 22);
        }

        #[test]
        fn two_nodes_2() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_connection(Connection::new(0, 1, 10));

            let city_0_consumption = Consumer::new(0, Function::new(0, vec![13, 13, 12, 11, 13]));
            let city_0_production = Producer::new(0, Function::new(0, vec![11, 11, 12, 13, 13]));
            let city_1_consumption = Consumer::new(1, Function::new(20, vec![3, 3, 2, 1, 1]));
            let city_1_production = Producer::new(1, Function::new(20, vec![1, 1, 2, 3, 3]));

            let mut market_base = Market::new(geography);
            market_base.add_consumer(&city_0_consumption);
            market_base.add_producer(&city_0_production);
            market_base.add_consumer(&city_1_consumption);
            market_base.add_producer(&city_1_production);
            let mut market = Market {
                geography: market_base.geography,
                cities: market_base.cities,
                prices: BTreeMap::from([(0, 2), (1, 22)]),
            };

            market.update_prices();
            let prices = market.get_prices();

            println!("{} {}", prices[&0], prices[&1]);

            assert_eq!(prices[&1] - prices[&0], 10);
        }

        #[test]
        fn three_node_1() {
            let mut geography = Geography::new();
            geography.add_city(City::new(0, "city 0".to_string()));
            geography.add_city(City::new(1, "city 1".to_string()));
            geography.add_city(City::new(2, "city 2".to_string()));
            geography.add_connection(Connection::new(0, 1, 3));
            geography.add_connection(Connection::new(1, 2, 2));

            let city_0_consumption = Consumer::new(0, Function::new(0, vec![10]));
            let city_0_production = Producer::new(0, Function::new(0, vec![10]));
            let city_1_consumption = Consumer::new(1, Function::new(5, vec![10]));
            let city_1_production = Producer::new(1, Function::new(5, vec![10]));
            let city_2_consumption = Consumer::new(1, Function::new(8, vec![10]));
            let city_2_production = Producer::new(1, Function::new(8, vec![10]));

            let mut market_base = Market::new(geography);
            market_base.add_consumer(&city_0_consumption);
            market_base.add_producer(&city_0_production);
            market_base.add_consumer(&city_1_consumption);
            market_base.add_producer(&city_1_production);
            market_base.add_consumer(&city_2_consumption);
            market_base.add_producer(&city_2_production);
            let mut market = Market {
                geography: market_base.geography,
                cities: market_base.cities,
                prices: BTreeMap::from([(0, 0), (1, 5), (2, 8)]),
            };

            market.update_prices();
            let prices = market.get_prices();

            println!("{} {}", prices[&0], prices[&1]);

            assert_eq!(prices[&1] - prices[&0], 3);
            assert_eq!(prices[&2] - prices[&1], 2);
        }
    }
}

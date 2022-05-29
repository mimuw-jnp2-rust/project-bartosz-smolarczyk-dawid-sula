use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::function::Function;
use crate::economy::geography::CityId;
use crate::economy::geography::Geography;
use crate::util::types::Value;
use mcmf::{Capacity, Cost, GraphBuilder, Vertex};
use std::collections::HashMap;

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
    cities: HashMap<CityId, CityData>,

    /// Currect prices in cities. All values are 0 if they haven't
    /// been calculated yet.
    prices: HashMap<CityId, Value>,
}

impl Market {
    pub fn new(geography: Geography) -> Market {
        let cities: HashMap<CityId, CityData> = geography
            .get_cities()
            .into_iter()
            .map(|x| (x.get_id(), CityData::new()))
            .collect();
        let prices: HashMap<CityId, Value> = geography
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

    pub fn get_prices(&self) -> &HashMap<CityId, Value> {
        &self.prices
    }

    fn calculate_groups(
        &self,
        pos: CityId,
        group_id: CityId,
        group_diff: Value,
        groups: &mut HashMap<CityId, (CityId, Value)>,
    ) {
        if !groups.contains_key(&pos) {
            return;
        }
        groups.get_mut(&pos).unwrap().0 = group_id;
        groups.get_mut(&pos).unwrap().1 = group_diff;

        let connections = self.geography.get_connections();
        for conn in connections[pos] {
            let id_from = conn.get_from_id();
            let id_to = conn.get_to_id();
            let cost = conn.get_cost();

            let price_from = self.prices[&id_from];
            let price_to = self.prices[&id_to];

            if (price_from - price_to).abs() >= cost {
                self.calculate_groups(id_to, group_id, group_diff + price_from - price_to, groups)
            }
        }
    }

    pub fn update_prices(&mut self) {
        // Map id -> (group_id, price_compared_to_groups_base).
        let mut groups: HashMap<CityId, (CityId, Value)> = HashMap::new();
        for i in self.cities.keys() {
            self.calculate_groups(*i, *i, 0, &mut groups);
        }

        // Map group_id -> [(id, price_compared_to_groups_base)].
        let mut group_lists: HashMap<CityId, Vec<(CityId, Value)>> =
            self.cities.keys().map(|x| (*x, vec![])).collect();
        for city in groups {
            group_lists
                .get_mut(&city.1 .0)
                .unwrap()
                .push((city.0, city.1 .1));
        }

        for group in group_lists {
            let mut demand = Function::zero();
            let mut supply = Function::zero();

            for (city_id, price_diff) in &group.1 {
                let city = &self.cities[city_id];
                demand.add_function(city.get_demand().clone().shift(*price_diff));
                supply.add_function(city.get_supply().clone().shift(*price_diff));
            }

            let price = demand.intersect(&supply);
            for (city_id, price_diff) in &group.1 {
                *self.prices.get_mut(city_id).unwrap() = price + price_diff;
            }
        }
    }
}

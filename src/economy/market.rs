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
            demand: Function::new(0, vec![]),
            supply: Function::new(0, vec![]),
        }
    }

    pub fn get_demand(&self) -> &Function {
        &self.demand
    }

    fn add_demand(&mut self, demand: &Function) {
        self.demand.add_function(demand)
    }

    fn substract_demand(&mut self, demand: &Function) {
        self.demand.substract_function(demand)
    }

    pub fn get_supply(&self) -> &Function {
        &self.supply
    }

    fn add_supply(&mut self, supply: &Function) {
        self.supply.add_function(supply)
    }

    fn substract_supply(&mut self, supply: &Function) {
        self.supply.substract_function(supply)
    }
}

#[derive(Debug)]
pub struct Market {
    geography: Geography,
    cities: HashMap<CityId, CityData>,
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

    pub fn update_prices(&self) {
        
    }
}

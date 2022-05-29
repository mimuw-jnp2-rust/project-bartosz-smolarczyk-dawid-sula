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
    fn new(demand: Function, supply: Function) -> CityData {
        CityData { demand, supply }
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
}

impl Market {
    pub fn new(geography: Geography) -> Market {
        Market {
            geography,
            cities: HashMap::new(),
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

    pub fn calculate_prices(&self) -> HashMap<CityId, Value> {
        let mut builder = GraphBuilder::<CityId>::new();

        for con in self.geography.get_connections().into_iter().flatten() {
            let from = con.get_from_id();
            let to = con.get_to_id();
            let capacity = con.get_max_volume();
            let cost = con.get_cost();
            builder.add_edge(from, to, Capacity(capacity), Cost(cost));
        }
        for city in self.geography.get_cities() {
            let supply = &self.cities[&city.get_id()].get_supply();
            let min_price = supply.arg_min();
            let max_price = supply.arg_max();
            let mut prev_volume = 0;
            for seg in supply.value_at_interval(min_price, max_price) {
                let (seg_min_price, _, seg_volume) = seg;
                builder.add_edge(
                    Vertex::Source,
                    city.get_id(),
                    Capacity(seg_volume - prev_volume),
                    Cost(seg_min_price),
                );
                prev_volume = seg_volume
            }

            let demand = &self.cities[&city.get_id()].get_demand();
            let min_price = demand.arg_min();
            let max_price = demand.arg_max();
            prev_volume = 0;
            for seg in demand
                .value_at_interval(min_price, max_price)
                .into_iter()
                .rev()
            {
                let (seg_min_price, _, seg_volume) = seg;
                builder.add_edge(
                    city.get_id(),
                    Vertex::Sink,
                    Capacity(seg_volume - prev_volume),
                    Cost(seg_min_price),
                );
            }
        }

        todo!()
    }
}

use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::function::Function;
use crate::economy::geography::CityId;
use crate::economy::geography::Geography;
use crate::util::types::Value;
use mcmf::{Capacity, Cost, GraphBuilder, Vertex};

#[derive(Debug)]
pub struct Market {
    geography: Geography,
    demand: Vec<Function>,
    supply: Vec<Function>,
}

impl Market {
    pub fn new() -> Market {
        Market {
            geography: Geography::new(),
            demand: vec![],
            supply: vec![],
        }
    }

    pub fn add_producer(&mut self, prod: &Producer) {
        self.supply[prod.get_city()].add_function(prod.get_supply())
    }

    pub fn remove_producer(&mut self, prod: &Producer) {
        self.supply[prod.get_city()].substract_function(prod.get_supply())
    }

    pub fn add_consumer(&mut self, cons: &Consumer) {
        self.demand[cons.get_city()].add_function(cons.get_demand())
    }

    pub fn remove_consumer(&mut self, cons: &Consumer) {
        self.demand[cons.get_city()].substract_function(cons.get_demand())
    }

    pub fn resolve_prices(&self) -> Vec<Value> {
        let mut builder = GraphBuilder::<CityId>::new();

        for con in self.geography.get_connections().iter().flatten() {
            let from = con.get_from_id();
            let to = con.get_to_id();
            let capacity = con.get_max_volume();
            let cost = con.get_cost();
            builder.add_edge(from, to, Capacity(capacity), Cost(cost));
        }
        for city in self.geography.get_cities() {
            let supply = &self.supply[city.get_id()];
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

            let demand = &self.demand[city.get_id()];
            let min_price = demand.arg_min();
            let max_price = demand.arg_max();
            prev_volume = 0;
            let mut prev_value = 0;
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

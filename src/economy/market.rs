use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::util::types::Value;
use crate::economy::function::Function;
use crate::economy::geography::Geography;
use crate::economy::geography::CityId;
use mcmf::{GraphBuilder, Vertex, Cost, Capacity};

#[derive(Debug)]
pub struct Market {
    geography: Geography,
    demand: Vec<Function>,
    supply: Vec<Function>
}

impl Market {
    pub fn new() -> Market {
        Market{geography: Geography::new(), demand: vec!{}, supply: vec!{}}
    }

    pub fn add_producer(&mut self, prod: &Producer) {
        self.supply[prod.getCity()].add_function(prod.getSupply())
    }

    pub fn remove_producer(&mut self, prod: &Producer) {
        self.supply[prod.getCity()].substract_function(prod.getSupply())
    }

    pub fn addConsumer(&mut self, cons: &Consumer) {
        self.demand[cons.getCity()].add_function(cons.getDemand())
    }
    
    pub fn removeConsumer(&mut self, cons: &Consumer) {
        self.demand[cons.getCity()].substract_function(cons.getDemand())
    }

    pub fn resolvePrices(&self) -> Vec<Value> {
        let mut builder = GraphBuilder::<CityId>::new();

        for con in self.geography.getConnections().into_iter().flatten() {
            let from = con.getFromId();
            let to = con.getToId();
            let capacity = con.getMaxVolume();
            let cost = con.getCost();
            builder.add_edge(from, to, Capacity(capacity), Cost(cost));
        }
        for city in self.geography.getCities() {
            let supply = &self.supply[city.getId()];
            let minPrice = supply.arg_min();
            let maxPrice = supply.arg_max();
            let mut prevVolume = 0;
            for seg in supply.value_at_interval(minPrice, maxPrice) {
                let (segMinPrice, _, segVolume) = seg;
                builder.add_edge(Vertex::Source, city.getId(), Capacity(segVolume - prevVolume), Cost(segMinPrice));
                prevVolume = segVolume
            }

            let demand = &self.demand[city.getId()];
            let minPrice = demand.arg_min();
            let maxPrice = demand.arg_max();
            prevVolume = 0;
            let mut prevValue = 0;
            for seg in demand.value_at_interval(minPrice, maxPrice).into_iter().rev() {
                let (segMinPrice, _, segVolume) = seg;
                builder.add_edge(city.getId(), Vertex::Sink, Capacity(segVolume - prevVolume), Cost(segMinPrice));
            }
        }

        todo!()
    }
}

use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::geography::City;
use crate::economy::geography::CityId;
use crate::economy::geography::Connection;
use crate::economy::geography::Geography;
use crate::economy::market::Market;
use crate::util::types::Value;
use std::collections::BTreeMap;

pub struct SimulationBuilder {
    geography: Geography,
}

impl SimulationBuilder {
    pub fn new() -> SimulationBuilder {
        SimulationBuilder {
            geography: Geography::new(),
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.geography.add_city(city)
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.geography.add_connection(connection)
    }

    pub fn build(self) -> Simulation {
        Simulation::new(self.geography)
    }
}

#[derive(Debug)]
pub struct Simulation {
    pub market: Market,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl Simulation {
    fn new(geography: Geography) -> Simulation {
        Simulation {
            market: Market::new(geography),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn add_producer(&mut self, producer: Producer) {
        self.market.add_producer(&producer);
        self.producers.push(producer)
    }

    pub fn add_consumer(&mut self, consumer: Consumer) {
        self.market.add_consumer(&consumer);
        self.consumers.push(consumer)
    }

    fn simulate_round(&mut self) {
        self.market.update_prices();
        for prod in &mut self.producers {
            prod.update(&mut self.market)
        }
        for cons in &mut self.consumers {
            cons.update(&mut self.market)
        }
    }

    pub fn calculate_prices(&mut self) -> &BTreeMap<CityId, Value> {
        self.simulate_round();
        self.market.get_prices()
    }
}

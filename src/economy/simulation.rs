use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::geography::City;
use crate::economy::geography::CityId;
use crate::economy::geography::Connection;
use crate::economy::geography::Geography;
use crate::economy::market::Market;
use crate::util::types::Value;
use std::collections::HashMap;

pub struct Simulation {
    geography: Geography,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            geography: Geography::new(),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.geography.add_city(city)
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.geography.add_connection(connection)
    }

    pub fn add_producer(&mut self, producer: Producer) {
        self.producers.push(producer)
    }

    pub fn add_consumer(&mut self, consumer: Consumer) {
        self.consumers.push(consumer)
    }

    pub fn calculate_prices(&mut self) -> HashMap<CityId, Value> {
        let mut prices = HashMap::new();

        for i in 1..10 {
            if i != 1 {
                for prod in &mut self.producers {
                    prod.update(&self.geography, &prices)
                }
            }

            let mut market = Market::new(self.geography.clone());
            for prod in &self.producers {
                market.add_producer(prod)
            }
            for cons in &self.consumers {
                market.add_consumer(cons)
            }
            prices = market.calculate_prices();
        }
        prices
    }
}

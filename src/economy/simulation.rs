use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::economy::entity::Consumer;
use crate::economy::entity::Producer;
use crate::economy::geography::City;
use crate::economy::geography::CityId;
use crate::economy::geography::Connection;
use crate::economy::geography::Geography;
use crate::economy::market::Market;
use crate::util::types::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationBuilder {
    turns: usize,
    cities: Vec<City>,
    connections: Vec<Connection>,
    prices: BTreeMap<CityId, Value>,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl SimulationBuilder {
    pub fn new() -> SimulationBuilder {
        SimulationBuilder {
            turns: 0,
            cities: vec![],
            connections: vec![],
            prices: BTreeMap::new(),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<SimulationBuilder, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let simulation_builder = serde_json::from_reader(reader)?;
        Ok(simulation_builder)
    }

    pub fn build(self) -> Simulation {
        let mut geography = Geography::new();
        for city in self.cities {
            geography.add_city(city);
        }
        for connection in self.connections {
            geography.add_connection(connection);
        }

        let mut simulation = Simulation::new(self.turns, geography);
        for (city_id, price) in self.prices {
            simulation.change_price(city_id, price);
        }
        for producer in self.producers {
            simulation.add_producer(producer);
        }
        for consumer in self.consumers {
            simulation.add_consumer(consumer);
        }

        simulation
    }
}

#[derive(Debug)]
pub struct Simulation {
    turns: usize,
    pub market: Market,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl Simulation {
    fn new(turns: usize, geography: Geography) -> Simulation {
        Simulation {
            turns,
            market: Market::new(geography),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn change_price(&mut self, city_id: CityId, price: Value) {
        self.market.change_price(&city_id, &price);
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

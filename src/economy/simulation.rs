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

use super::types::Price;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationBuilder {
    turns: usize,
    cities: Vec<City>,
    connections: Vec<Connection>,
    initial_prices: Vec<(CityId, Price)>,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl SimulationBuilder {
    pub fn new() -> SimulationBuilder {
        SimulationBuilder {
            turns: 0,
            cities: vec![],
            connections: vec![],
            initial_prices: vec![],
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
        for init in self.initial_prices {
            simulation.change_price(init.0, init.1);
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

    fn change_price(&mut self, city_id: CityId, price: Price) {
        self.market.change_price(&city_id, &price);
    }

    fn add_producer(&mut self, producer: Producer) {
        self.market.add_producer(&producer);
        self.producers.push(producer)
    }

    fn add_consumer(&mut self, consumer: Consumer) {
        self.market.add_consumer(&consumer);
        self.consumers.push(consumer)
    }

    fn simulate_turn(&mut self) {
        self.market.update_prices();
        for prod in &mut self.producers {
            prod.update(&mut self.market)
        }
        for cons in &mut self.consumers {
            cons.update(&mut self.market)
        }
    }

    pub fn run(&mut self) -> SimulationResult {
        let mut result = SimulationResult::new(self);
        for _ in 0..self.turns {
            self.simulate_turn();
            for (city_id, price) in self.market.prices() {
                let name = self.market.geography().cities().get(city_id).unwrap().name.clone();
                result.prices.get_mut(&name).unwrap().push(price);
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct SimulationResult {
    prices: BTreeMap<String, Vec<Option<Price>>>,
}

impl SimulationResult {
    fn new(simulation: &Simulation) -> SimulationResult {
        let mut initial_prices = BTreeMap::new();
        for city in simulation.market.geography().cities() {
            let price = *simulation.market.prices().get(&city.id).unwrap();
            initial_prices.insert(city.name.clone(), vec![price]);
        }
        SimulationResult {
            prices: initial_prices,
        }
    }
}

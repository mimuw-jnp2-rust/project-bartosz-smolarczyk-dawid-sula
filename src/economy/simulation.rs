use std::collections::BTreeMap;
use std::error::Error;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use plotters::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct Simulation {
    turns: usize,
    pub market: Market,
    producers: Vec<Producer>,
    consumers: Vec<Consumer>,
}

impl Simulation {
    fn new(turns: usize, geography: Geography, prices: BTreeMap<CityId, Price>) -> Simulation {
        Simulation {
            turns,
            market: Market::new(geography, prices),
            producers: vec![],
            consumers: vec![],
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Simulation, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let simulation_builder: SimulationBuilder = serde_json::from_reader(reader)?;
    
        let mut geography = Geography::new();
        for city in simulation_builder.cities {
            geography.add_city(city);
        }
        for connection in simulation_builder.connections {
            geography.add_connection(connection);
        }
    
        let mut simulation = Simulation::new(
            simulation_builder.turns,
            geography,
            simulation_builder.initial_prices.into_iter().collect(),
        );
        for producer in simulation_builder.producers {
            simulation.add_producer(producer);
        }
        for consumer in simulation_builder.consumers {
            simulation.add_consumer(consumer);
        }
    
        Ok(simulation)
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
        self.market.simulate(1);
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
                let name = self
                    .market
                    .geography()
                    .cities()
                    .get(city_id)
                    .unwrap()
                    .name
                    .clone();
                result.prices.get_mut(&name).unwrap().push(price);
            }
        }
        result
    }

    pub fn plot(&mut self, output_file: &str) -> Result<(), Box<dyn Error>> {
        const SIZE_X: u32 = 1024; // be sensible here
        const SIZE_Y: u32 = 768; // be sensible here too
        const MIN_X: f32 = 0.0;
        const MAX_X: f32 = 10.0;
        const AVG_X: f32 = (MAX_X + MIN_X) / 2.0;
        const DEV_X: f32 = (MAX_X - MIN_X) / 2.0;
        const MIN_Y: f32 = 0.0;
        const MAX_Y: f32 = 10.0;
        const AVG_Y: f32 = (MAX_Y + MIN_Y) / 2.0;
        const DEV_Y: f32 = (MAX_Y - MIN_Y) / 2.0;
        let root_area = BitMapBackend::new(output_file, (SIZE_X, SIZE_Y)).into_drawing_area();

        root_area.fill(&WHITE)?;

        let root_area = root_area.titled("TITLE", ("sans-serif", 60))?;

        let x_axis = (MIN_X..MAX_X).step((MAX_X - MIN_X) / 100.0);

        let mut chart_builder = ChartBuilder::on(&root_area)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Subtitle", ("sans-serif", 40))
            .build_cartesian_2d(MIN_X..MAX_X, MIN_Y..MAX_Y)?;

        chart_builder
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .disable_mesh()
            .draw()?;

        chart_builder
            .draw_series(LineSeries::new(
                x_axis
                    .values()
                    .map(|x| (x, (x / PI - PI / 2.0).sin() * DEV_Y + DEV_Y)),
                &RED,
            ))?
            .label("Supply")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart_builder
            .draw_series(LineSeries::new(
                x_axis.values().map(|x| (x, (x / PI).cos() * DEV_Y + DEV_Y)),
                &BLUE,
            ))?
            .label("Demand")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart_builder
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

        Ok(())
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

//! Input reading and output writing functions

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use text_io::read;

use crate::economy::entity::{Consumer, Producer};
use crate::economy::function::Function;
use crate::economy::geography::{City, CityId, Connection};
use crate::economy::simulation::{Simulation, SimulationBuilder};
use crate::util::types::{Value, Volume};

pub trait Writer {
    fn to_file(fd: &File, data: &Self);
}

impl Writer for Simulation {
    fn to_file(mut fd: &File, data: &Simulation) {
        fd.write_all("RESULTS\n\n".as_bytes())
            .expect("Error in write");
        let prices = data.market.get_prices();
        for (city_id, price) in prices {
            let record: String = data
                .market
                .get_geography()
                .cities
                .get(city_id)
                .unwrap()
                .name
                .clone()
                + ": "
                + &price.to_string()
                + "\n";
            fd.write_all(record.as_bytes()).expect("Error in write");
        }
    }
}

pub trait Reader {
    fn from_file(fd: &File) -> Self;
}

fn check_header(fd: &File, expected: String) {
    let header: String = Reader::from_file(fd);
    if expected != header {
        eprintln!("{} not found", expected);
        std::process::exit(1);
    }
}

impl Reader for usize {
    fn from_file(fd: &File) -> usize {
        let mut iter = fd.bytes().map(|ch| ch.unwrap());
        let res: usize = read!("{}", iter);
        res
    }
}

impl Reader for i32 {
    fn from_file(fd: &File) -> i32 {
        let mut iter = fd.bytes().map(|ch| ch.unwrap());
        let res: i32 = read!("{}", iter);
        res
    }
}

impl Reader for String {
    fn from_file(fd: &File) -> String {
        let mut iter = fd.bytes().map(|ch| ch.unwrap());
        let res: String = read!("{}", iter);
        res
    }
}

impl Reader for City {
    fn from_file(fd: &File) -> City {
        let id: CityId = Reader::from_file(fd);
        let name: String = Reader::from_file(fd);
        City::new(id, name)
    }
}

impl Reader for Connection {
    fn from_file(fd: &File) -> Connection {
        let id_from: CityId = Reader::from_file(fd);
        let id_to: CityId = Reader::from_file(fd);
        let cost: Value = Reader::from_file(fd);
        Connection::new(id_from, id_to, cost)
    }
}

impl Reader for Function {
    fn from_file(fd: &File) -> Function {
        let arg_min: Value = Reader::from_file(fd);
        let mut values_cnt: usize = Reader::from_file(fd);
        let mut values: Vec<Volume> = vec![];
        while values_cnt > 0 {
            let value: Volume = Reader::from_file(fd);
            values.push(value);
            values_cnt -= 1;
        }
        Function::new(arg_min, values)
    }
}

impl Reader for Producer {
    fn from_file(fd: &File) -> Producer {
        let city: CityId = Reader::from_file(fd);
        let production_costs: Function = Reader::from_file(fd);
        Producer::new(city, production_costs)
    }
}

impl Reader for Consumer {
    fn from_file(fd: &File) -> Consumer {
        let city: CityId = Reader::from_file(fd);
        let usefulness: Function = Reader::from_file(fd);
        Consumer::new(city, usefulness)
    }
}

impl Reader for Simulation {
    fn from_file(fd: &File) -> Simulation {
        let mut simulation_builder = SimulationBuilder::new();

        check_header(fd, String::from("GEOGRAPHY"));
        check_header(fd, String::from("Cities:"));

        let mut cities_cnt: usize = Reader::from_file(fd);
        while cities_cnt > 0 {
            let city: City = Reader::from_file(fd);
            simulation_builder.add_city(city);
            cities_cnt -= 1;
        }

        check_header(fd, String::from("Connections:"));

        let mut connections_cnt: usize = Reader::from_file(fd);
        while connections_cnt > 0 {
            let connection: Connection = Reader::from_file(fd);
            simulation_builder.add_connection(connection);
            connections_cnt -= 1;
        }

        let mut simulation: Simulation = simulation_builder.build();

        check_header(fd, String::from("ECONOMY"));
        check_header(fd, String::from("Prices:"));

        let mut prices_cnt: usize = Reader::from_file(fd);
        while prices_cnt > 0 {
            let city_id: CityId = Reader::from_file(fd);
            let price: Value = Reader::from_file(fd);
            simulation.change_price(city_id, price);
            prices_cnt -= 1;
        }

        check_header(fd, String::from("Producers:"));

        let mut producers_cnt: usize = Reader::from_file(fd);
        while producers_cnt > 0 {
            let producer: Producer = Reader::from_file(fd);
            simulation.add_producer(producer);
            producers_cnt -= 1;
        }

        check_header(fd, String::from("Consumers:"));

        let mut consumers_cnt: usize = Reader::from_file(fd);
        while consumers_cnt > 0 {
            let consumer: Consumer = Reader::from_file(fd);
            simulation.add_consumer(consumer);
            consumers_cnt -= 1;
        }

        simulation
    }
}

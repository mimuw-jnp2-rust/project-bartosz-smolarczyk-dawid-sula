use crate::util::types::Value;
use crate::util::types::Volume;
use std::collections::HashMap;

pub type CityId = usize;

#[derive(Clone, Debug)]
pub struct City {
    id: CityId,
    name: String,
}

impl City {
    pub fn new(id: CityId, name: String) -> City {
        City { id, name }
    }

    pub fn get_id(&self) -> CityId {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone, Debug)]
pub struct Connection {
    id_from: CityId,
    id_to: CityId,
    cost: Value,
}

impl Connection {
    pub fn new(id_from: CityId, id_to: CityId, cost: Value, max_volume: Volume) -> Connection {
        Connection {
            id_from,
            id_to,
            cost,
        }
    }

    pub fn get_from_id(&self) -> CityId {
        self.id_from
    }

    pub fn get_to_id(&self) -> CityId {
        self.id_to
    }

    pub fn get_cost(&self) -> Value {
        self.cost
    }
}

#[derive(Clone, Debug)]
pub struct Geography {
    cities: HashMap<CityId, City>,
    connections: HashMap<CityId, Vec<Connection>>,
}

impl Geography {
    pub fn new() -> Geography {
        Geography {
            cities: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.cities.insert(city.get_id(), city);
    }

    pub fn add_connection(&mut self, connection: Connection) {
        let id_from = connection.get_from_id();
        let id_to = connection.get_to_id();
        self.connections.get_mut(&id_from).unwrap().push(connection);
    }

    pub fn get_cities(&self) -> Vec<&City> {
        Vec::from_iter(self.cities.values())
    }

    pub fn get_connections(&self) -> Vec<&Vec<Connection>> {
        Vec::from_iter(self.connections.values())
    }
}

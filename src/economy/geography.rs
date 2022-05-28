use crate::util::types::Value;
use crate::util::types::Volume;

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
    max_volume: Volume,
}

impl Connection {
    pub fn new(id_from: CityId, id_to: CityId, cost: Value, max_volume: Volume) -> Connection {
        Connection {
            id_from,
            id_to,
            cost,
            max_volume,
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

    pub fn get_max_volume(&self) -> Volume {
        self.max_volume
    }
}

#[derive(Clone, Debug)]
pub struct Geography {
    cities: Vec<City>,
    connections: Vec<Vec<Connection>>,
}

impl Geography {
    pub fn new() -> Geography {
        Geography {
            cities: vec![],
            connections: vec![],
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.cities.push(city)
    }

    pub fn add_connection(&mut self, connection: Connection) {
        let id_from = connection.get_from_id();
        let id_to = connection.get_to_id();
        self.connections[id_from].push(connection.clone());
        self.connections[id_to].push(connection);
    }

    pub fn get_cities(&self) -> &Vec<City> {
        &self.cities
    }

    pub fn get_connections(&self) -> &Vec<Vec<Connection>> {
        &self.connections
    }
}

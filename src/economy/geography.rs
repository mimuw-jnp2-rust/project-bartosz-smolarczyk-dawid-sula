use crate::util::types::Volume;
use crate::util::types::Value;

pub type CityId = usize;

#[derive(Clone, Debug)]
pub struct City {
    id: CityId,
    name: String
}

impl City {
    pub fn new(id: CityId, name: String) -> City {
        City{id, name}
    }

    pub fn getId(&self) -> CityId {
        self.id
    }

    pub fn getName(&self) -> &String {
        &self.name
    }
}


#[derive(Clone, Debug)]
pub struct Connection {
    id_from: CityId,
    id_to: CityId,
    cost: Value,
    max_volume: Volume
}

impl Connection {
    pub fn new(id_from: CityId, id_to: CityId, cost: Value, max_volume: Volume) -> Connection {
        Connection{id_from, id_to, cost, max_volume}
    }

    pub fn getFromId(&self) -> CityId {
        self.id_from
    }

    pub fn getToId(&self) -> CityId {
        self.id_to
    }

    pub fn getCost(&self) -> Value {
        self.cost
    }

    pub fn getMaxVolume(&self) -> Volume {
        self.max_volume
    }
}


#[derive(Clone, Debug)]
pub struct Geography {
    cities: Vec<City>,
    connections: Vec<Vec<Connection>>
}

impl Geography {
    pub fn new() -> Geography {
        Geography{cities: vec!{}, connections: vec!{}}
    }

    pub fn addCity(&mut self, city: City) {
        self.cities.push(city)
    }

    pub fn addConnection(&mut self, connection: Connection) {
        let id_from = connection.getFromId();
        let id_to = connection.getToId();
        self.connections[id_from].push(connection.clone());
        self.connections[id_to].push(connection);
    }

    pub fn getCities(&self) -> &Vec<City> {
        &self.cities
    }

    pub fn getConnections(&self) -> &Vec<Vec<Connection>> {
        &self.connections
    }
}
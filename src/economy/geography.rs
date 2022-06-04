use std::collections::BTreeMap;

use super::types::Price;

pub type CityId = usize;

#[derive(Clone, Debug)]
pub struct City {
    pub id: CityId,
    pub name: String,
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
    cost: Price,
}

impl Connection {
    pub fn new(id_from: CityId, id_to: CityId, cost: Price) -> Connection {
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

    pub fn get_cost(&self) -> Price {
        self.cost
    }
}

#[derive(Clone, Debug)]
pub struct Geography {
    pub cities: BTreeMap<CityId, City>,
    pub connections: BTreeMap<CityId, Vec<Connection>>,
}

impl Geography {
    pub fn new() -> Geography {
        Geography {
            cities: BTreeMap::new(),
            connections: BTreeMap::new(),
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.connections.insert(city.get_id(), vec![]);
        self.cities.insert(city.get_id(), city);
    }

    pub fn add_connection(&mut self, connection: Connection) {
        let id_from = connection.get_from_id();
        let id_to = connection.get_to_id();
        let rev_connection = Connection::new(id_to, id_from, connection.get_cost());

        self.connections.get_mut(&id_from).unwrap().push(connection);
        self.connections
            .get_mut(&id_to)
            .unwrap()
            .push(rev_connection);
    }

    pub fn get_cities(&self) -> Vec<&City> {
        Vec::from_iter(self.cities.values())
    }

    pub fn get_connections(&self) -> Vec<&Vec<Connection>> {
        Vec::from_iter(self.connections.values())
    }
}

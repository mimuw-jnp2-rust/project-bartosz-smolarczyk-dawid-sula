use crate::economy::function::Function;
use crate::economy::geography::CityId;

#[derive(Clone, Debug)]
pub struct Producer {
    city: CityId,
    production_costs: Function,
}

impl Producer {
    pub fn new(city: CityId, production_costs: Function) -> Producer {
        Producer {
            city,
            production_costs,
        }
    }

    pub fn get_city(&self) -> CityId {
        self.city
    }

    pub fn get_supply(&self) -> Function {
        self.production_costs.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Consumer {
    city: CityId,
    usefulness: Function,
}

impl Consumer {
    pub fn new(city: CityId, usefulness: Function) -> Consumer {
        Consumer { city, usefulness }
    }

    pub fn get_city(&self) -> CityId {
        self.city
    }

    pub fn get_demand(&self) -> Function {
        self.usefulness.clone()
    }
}

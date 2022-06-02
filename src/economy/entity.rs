use crate::economy::function::Function;
use crate::economy::geography::CityId;
use crate::economy::market::Market;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
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

    pub fn get_supply(&self) -> &Function {
        &self.production_costs
    }

    pub fn update(&mut self, market: &mut Market) {
        // TODO: some sort of simple update.
    }
}

#[derive(Deserialize, Clone, Debug)]
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

    pub fn get_demand(&self) -> &Function {
        &self.usefulness
    }

    pub fn update(&mut self, market: &mut Market) {}
}

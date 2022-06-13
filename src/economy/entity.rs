use crate::economy::function::Demand;
use crate::economy::function::Supply;
use crate::economy::geography::CityId;
use crate::economy::market::Market;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Producer {
    city: CityId,
    production_costs: Supply,
}

impl Producer {
    #[allow(dead_code)]
    pub fn new(city: CityId, production_costs: Supply) -> Producer {
        Producer {
            city,
            production_costs,
        }
    }

    pub fn city(&self) -> CityId {
        self.city
    }

    pub fn supply(&self) -> &Supply {
        &self.production_costs
    }

    #[allow(dead_code)]
    pub fn update(&mut self, _market: &mut Market) {
        // Place left for possible extension.
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Consumer {
    city: CityId,
    usefulness: Demand,
}

impl Consumer {
    #[allow(dead_code)]
    pub fn new(city: CityId, usefulness: Demand) -> Consumer {
        Consumer { city, usefulness }
    }

    pub fn city(&self) -> CityId {
        self.city
    }

    pub fn demand(&self) -> &Demand {
        &self.usefulness
    }

    #[allow(dead_code)]
    pub fn update(&mut self, _market: &mut Market) {
        // Place left for possible extension.
    }
}

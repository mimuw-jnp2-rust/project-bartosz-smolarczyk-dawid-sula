use crate::economy::function::Demand;
use crate::economy::function::Supply;
use crate::economy::geography::CityId;
use crate::economy::market::Market;

#[derive(Clone, Debug)]
pub struct Producer {
    city: CityId,
    production_costs: Supply,
}

impl Producer {
    pub fn new(city: CityId, production_costs: Supply) -> Producer {
        Producer {
            city,
            production_costs,
        }
    }

    pub fn get_city(&self) -> CityId {
        self.city
    }

    pub fn get_supply(&self) -> &Supply {
        &self.production_costs
    }

    pub fn update(&mut self, market: &mut Market) {
        // TODO: some sort of simple update.
    }
}

#[derive(Clone, Debug)]
pub struct Consumer {
    city: CityId,
    usefulness: Demand,
}

impl Consumer {
    pub fn new(city: CityId, usefulness: Demand) -> Consumer {
        Consumer { city, usefulness }
    }

    pub fn get_city(&self) -> CityId {
        self.city
    }

    pub fn get_demand(&self) -> &Demand {
        &self.usefulness
    }

    pub fn update(&mut self, market: &mut Market) {}
}

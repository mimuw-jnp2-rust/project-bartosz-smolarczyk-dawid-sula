use serde::Deserialize;
use serde::Serialize;

use crate::economy::function::supply::Supply;
use crate::economy::function::ArgT;
use crate::economy::function::FunctionAbstract;
use crate::economy::function::FunctionNullable;
use crate::economy::function::ValueT;
use crate::economy::market::MarketState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Demand {
    function: FunctionNullable,
}

impl Demand {
    pub fn zero() -> Demand {
        Demand {
            function: FunctionNullable::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn new<I>(values: I) -> Demand
    where
        I: Iterator<Item = (ArgT, ValueT)>,
    {
        Demand {
            function: FunctionNullable::new(values),
        }
    }

    pub fn function(&self) -> &FunctionNullable {
        &self.function
    }

    pub fn intersect(&self, supply: &Supply) -> MarketState {
        match self.function.intersect(supply.function()) {
            Some((price, amount)) => MarketState::Equilibrium(price, amount, amount),
            None => {
                if self.function().right_value() > supply.function().right_value() {
                    MarketState::UnderSupply
                } else if self.function().left_value() < supply.function().left_value() {
                    MarketState::OverSupply
                } else {
                    MarketState::Undefined
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn intervals(&self) -> Vec<(ArgT, ValueT)> {
        self.function.intervals()
    }
}

impl FunctionAbstract for Demand {
    fn value(&self, arg: ArgT) -> ValueT {
        self.function.value(arg)
    }

    fn add_value(&mut self, value: ValueT) -> &mut Self {
        self.function.add_value(value);
        self
    }

    fn substract_value(&mut self, value: ValueT) -> &mut Self {
        self.function.substract_value(value);
        self
    }

    fn add_function(&mut self, fun: &Self) -> &mut Self {
        self.function.add_function(fun.function());
        self
    }

    fn substract_function(&mut self, fun: &Self) -> &mut Self {
        self.function.substract_function(fun.function());
        self
    }

    fn shift_right(&mut self, arg: ArgT) -> &mut Self {
        self.function.shift_right(arg);
        self
    }

    fn shift_left(&mut self, arg: ArgT) -> &mut Self {
        self.function.shift_left(arg);
        self
    }

    fn negate(&mut self) -> &mut Self {
        self.function.negate();
        self
    }
}

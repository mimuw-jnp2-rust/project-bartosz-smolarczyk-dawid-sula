use serde::Deserialize;
use serde::Serialize;

use crate::economy::function::supply::Supply;
use crate::economy::function::ArgT;
use crate::economy::function::Function;
use crate::economy::function::FunctionAbstract;
use crate::economy::function::ValueT;
use crate::economy::market::MarketState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Demand {
    function: Function,
}

impl Demand {
    pub fn zero() -> Demand {
        let function = Function::zero();
        Demand { function }
    }

    pub fn new<I>(values: I) -> Demand
    where
        I: Iterator<Item = (ArgT, ValueT)>,
    {
        let function = Function::new(values);
        Demand { function }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn intersect(&self, supply: &Supply) -> MarketState {
        match self.function.intersect(supply.function()) {
            Some((price, amount)) => MarketState::Equilibrium(price, amount, amount),
            None => {
                if self.function().max_value > supply.function().max_value {
                    MarketState::UnderSupply
                } else if self.function().min_value < supply.function().min_value {
                    MarketState::OverSupply
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl FunctionAbstract for Demand {
    fn value(&self, arg: ArgT) -> ValueT {
        self.function.value(arg)
    }

    fn add_value(&mut self, value: ValueT) -> &Self {
        self.function.add_value(value);
        self
    }

    fn substract_value(&mut self, value: ValueT) -> &Self {
        self.function.substract_value(value);
        self
    }
    fn add_function(&mut self, fun: &Self) -> &Self {
        self.function.add_function(fun.function());
        self
    }
    fn substract_function(&mut self, fun: &Self) -> &Self {
        self.function.substract_function(fun.function());
        self
    }
    fn shift_right(&mut self, arg: ArgT) -> &Self {
        self.function.shift_right(arg);
        self
    }
    fn shift_left(&mut self, arg: ArgT) -> &Self {
        self.function.shift_left(arg);
        self
    }
}

use serde::Deserialize;
use serde::Serialize;

use crate::economy::function::demand::Demand;
use crate::economy::function::ArgT;
use crate::economy::function::Function;
use crate::economy::function::FunctionAbstract;
use crate::economy::function::ValueT;
use crate::economy::market::MarketState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Supply {
    function: Function,
}

impl Supply {
    pub fn zero() -> Supply {
        let function = Function::zero();
        Supply { function }
    }

    pub fn new<I>(values: I) -> Supply
    where
        I: Iterator<Item = (ArgT, ValueT)>,
    {
        let function = Function::new(values);
        Supply { function }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn intersect(&self, demand: &Demand) -> MarketState {
        demand.intersect(self)
    }
}

impl FunctionAbstract for Supply {
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

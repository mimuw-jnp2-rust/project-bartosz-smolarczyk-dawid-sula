use crate::economy::{
    function::{ArgT, Demand, Function, Supply, ValueT},
    types::InnerValue,
};

fn prepare_values_for_function(values: Vec<(InnerValue, InnerValue)>) -> Vec<(ArgT, ValueT)> {
    values
        .iter()
        .map(|x| (ArgT::new(x.0), ValueT::new(x.1)))
        .collect()
}

pub fn make_function(values: Vec<(InnerValue, InnerValue)>) -> Function {
    Function::new(prepare_values_for_function(values).into_iter())
}

pub fn make_demand(values: Vec<(InnerValue, InnerValue)>) -> Demand {
    Demand::new(prepare_values_for_function(values).into_iter())
}

pub fn make_supply(values: Vec<(InnerValue, InnerValue)>) -> Supply {
    Supply::new(prepare_values_for_function(values).into_iter())
}

pub fn test_eq_arg(a: ArgT, b: ArgT) {
    let tolerance = ArgT::new(0.1);
    if (a - b).abs() < tolerance {
    } else {
        print!("Assertion failed: {} != {}\n", a.float(), b.float());
        assert!(false)
    }
}

pub fn test_eq_value(a: ValueT, b: ValueT) {
    let tolerance = ValueT::new(0.1);
    if (a - b).abs() < tolerance {
    } else {
        print!("Assertion failed: {} != {}\n", a.float(), b.float());
        assert!(false)
    }
}

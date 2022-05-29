//! Base representation of mathematical function.

use crate::util::types::Value;
use crate::util::types::Volume;
use std::cmp::max;
use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Function {
    arg_min: Value,
    values: Vec<Volume>,
}

impl Function {
    pub fn zero() -> Function {
        Function {
            arg_min: 0,
            values: vec![0],
        }
    }

    pub fn new(arg_min: Value, values: Vec<Volume>) -> Function {
        Function { arg_min, values }
    }

    pub fn arg_min(&self) -> Value {
        self.arg_min
    }

    pub fn arg_max(&self) -> Value {
        self.arg_min + (self.values.len() as Value)
    }

    pub fn value_at(&self, arg: Value) -> Volume {
        if arg < self.arg_min() {
            self.values[0]
        } else if arg >= self.arg_max() {
            self.values[self.values.len() - 1]
        } else {
            self.values[(arg - self.arg_min()) as usize]
        }
    }

    pub fn value_at_interval(&self, arg_min: Value, arg_max: Value) -> Vec<(Value, Value, Volume)> {
        let mut res = vec![];
        let res_min_arg = min(arg_min, self.arg_min());
        let res_max_arg = max(arg_max, self.arg_max());

        for i in res_min_arg..res_max_arg {
            res.push((i, i + 1, self.value_at(i)))
        }
        res
    }

    pub fn add_value(&mut self, val: Volume) -> &mut Self {
        for i in &mut self.values {
            *i += val
        }
        self
    }

    pub fn add_function(&mut self, fun: &Self) -> &mut Self {
        let new_min_arg = min(self.arg_min(), fun.arg_min());
        let new_max_arg = max(self.arg_max(), fun.arg_max());

        let mut new_values = vec![];

        for i in new_min_arg..new_max_arg {
            new_values.push(self.value_at(i) + fun.value_at(i))
        }

        self.arg_min = new_min_arg;
        self.values = new_values;
        self
    }

    pub fn substract_function(&mut self, fun: &Self) -> &mut Self {
        let new_min_arg = min(self.arg_min(), fun.arg_min());
        let new_max_arg = max(self.arg_max(), fun.arg_max());

        let mut new_values = vec![];

        for i in new_min_arg..new_max_arg {
            new_values.push(self.value_at(i) - fun.value_at(i))
        }

        self.arg_min = new_min_arg;
        self.values = new_values;
        self
    }

    pub fn shift(&mut self, val: Value) -> &mut Self {
        self.arg_min += val;
        self
    }

    pub fn intersect_with_demand(&self, fun: &Self) -> Value {
        let res_min = min(self.arg_min(), fun.arg_min());
        let res_max = max(self.arg_max(), fun.arg_max());

        if self.value_at(res_min) > fun.value_at(res_min) {
            return Value::MIN;
        }

        for i in res_min..res_max {
            if self.value_at(i) >= fun.value_at(i) {
                return i;
            }
        }
        Value::MAX
    }

    pub fn intersect_with_supply(&self, fun: &Self) -> Value {
        let res_min = min(self.arg_min(), fun.arg_min());
        let res_max = max(self.arg_max(), fun.arg_max());

        if self.value_at(res_min) < fun.value_at(res_min) {
            return Value::MIN;
        }

        for i in res_min..res_max {
            if self.value_at(i) <= fun.value_at(i) {
                return i;
            }
        }
        Value::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod ranges {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = Function::new(15, vec![1, 2, 3, 4, 5, 6]);
            assert_eq!(fun.arg_min(), 15);
            assert_eq!(fun.arg_max(), 21);
        }
    }

    #[cfg(test)]
    mod value_access {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = Function::new(0, vec![1, 2, 3, 4, 5, 6, 7, 8]);
            assert_eq!(fun.value_at(2), 3);
            assert_eq!(fun.value_at(4), 5);
        }

        #[test]
        fn basic_2() {
            let fun = Function::new(-5, vec![9, 5, 4, 6, 8, 2]);
            assert_eq!(fun.value_at(-3), 4);
            assert_eq!(fun.value_at(0), 2);
            assert_eq!(fun.value_at(-4), 5);
        }

        #[test]
        fn outside_access() {
            let fun = Function::new(7, vec![1, 5, 4, 2]);
            assert_eq!(fun.value_at(6), 1);
            assert_eq!(fun.value_at(54), 2);
        }
    }

    #[cfg(test)]
    mod interval_access {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = Function::new(2, vec![1, 2, 3]);
            assert_eq!(
                fun.value_at_interval(2, 5),
                vec!((2, 3, 1), (3, 4, 2), (4, 5, 3))
            );
        }
    }

    #[cfg(test)]
    mod modification {
        use super::*;

        #[test]
        fn add_value_1() {
            let mut fun = Function::new(6, vec![5, 4, 6, 8, 7]);
            fun.add_value(6);
            assert_eq!(fun.values, vec! {11, 10, 12, 14, 13});
        }

        #[test]
        fn add_value_2() {
            let mut fun = Function::new(-4, vec![1]);
            fun.add_value(-8);
            assert_eq!(fun.values, vec! {-7});
        }

        #[test]
        fn add_function_1() {
            let mut fun_1 = Function::new(5, vec![8, 7, 5, 4]);
            let fun_2 = Function::new(5, vec![1, 2, 5, 7]);
            fun_1.add_function(&fun_2);
            assert_eq!(fun_1.arg_min, 5);
            assert_eq!(fun_1.values, vec! {9, 9, 10, 11});
        }

        #[test]
        fn add_function_2() {
            let mut fun_1 = Function::new(2, vec![2, 3, 4]);
            let fun_2 = Function::new(2, vec![5, 8]);
            fun_1.add_function(&fun_2);
            assert_eq!(fun_1.arg_min, 2);
            assert_eq!(fun_1.values, vec! {7, 11, 12});
        }

        #[test]
        fn add_function_3() {
            let mut fun_1 = Function::new(1, vec![5, 7, 6]);
            let fun_2 = Function::new(-2, vec![6, 8, 4, 5, 3, 9, 5, 4]);
            fun_1.add_function(&fun_2);
            assert_eq!(fun_1.arg_min, -2);
            assert_eq!(fun_1.values, vec! {11, 13, 9, 10, 10, 15, 11, 10});
        }

        #[test]
        fn shift_1() {
            let mut fun_1 = Function::new(4, vec![3, 5]);
            fun_1.shift(-7);
            assert_eq!(fun_1.arg_min, -3);
        }
    }

    #[cfg(test)]
    mod intersection {
        use super::*;

        #[test]
        fn basic_1() {
            let fun_1 = Function::new(3, vec![4, 5, 6, 7, 8]);
            let fun_2 = Function::new(2, vec![2, 4, 6, 8, 10]);
            assert_eq!(fun_1.intersect_with_supply(&fun_2), 3);
        }

        #[test]
        fn basic_2() {
            let fun_1 = Function::new(1, vec![7, 6, 5, 4, 3, 2, 1]);
            let fun_2 = Function::new(1, vec![1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(fun_1.intersect_with_supply(&fun_2), 4);
        }

        #[test]
        fn outside_access() {
            let fun_1 = Function::new(1, vec![1, 2, 3, 3, 3, 3, 3]);
            let fun_2 = Function::new(1, vec![8, 8, 8, 8, 5, 3, 1]);
            assert_eq!(fun_1.intersect_with_demand(&fun_2), 6);
        }

        #[test]
        fn double_outside_access() {
            let fun_1 = Function::new(1, vec![1, 2]);
            let fun_2 = Function::new(10, vec![2, 3]);
            assert_eq!(fun_1.value_at(fun_1.intersect_with_demand(&fun_2)), 2);
            assert_eq!(fun_2.value_at(fun_1.intersect_with_demand(&fun_2)), 2);
        }
    }
}

use serde::Deserialize;
use serde::Serialize;
use std::cmp::max;
use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;

pub use demand::Demand;
pub use supply::Supply;

mod demand;

mod supply;

pub type ArgT = crate::economy::types::Price;
pub type ValueT = crate::economy::types::Volume;

pub trait FunctionAbstract {
    fn value(&self, arg: ArgT) -> ValueT;

    fn add_value(&mut self, value: ValueT) -> &mut Self;
    fn substract_value(&mut self, value: ValueT) -> &mut Self;

    fn add_function(&mut self, function: &Self) -> &mut Self;
    fn substract_function(&mut self, function: &Self) -> &mut Self;

    fn shift_right(&mut self, shift: ArgT) -> &mut Self;
    fn shift_left(&mut self, shift: ArgT) -> &mut Self;
}

#[derive(Clone, Debug)]
pub struct Function {
    min_arg: ArgT,
    min_value: ValueT,
    max_arg: ArgT,
    max_value: ValueT,
    intervals: BTreeMap<ArgT, ValueT>,
}

impl Function {
    pub fn zero() -> Function {
        let intervals = vec![(ArgT::new(0.), ValueT::new(0.))];
        Function::new(intervals.into_iter())
    }

    pub fn new<I>(values: I) -> Function
    where
        I: Iterator<Item = (ArgT, ValueT)>,
    {
        let intervals: BTreeMap<ArgT, ValueT> = values.collect();
        assert!(!intervals.is_empty());

        let (min_arg, min_value) = intervals.iter().next().unwrap();
        let (max_arg, max_value) = intervals.iter().next_back().unwrap();

        Function {
            min_arg: *min_arg,
            min_value: *min_value,
            max_arg: *max_arg,
            max_value: *max_value,
            intervals: intervals,
        }
    }

    fn lower_bound(&self, arg: ArgT) -> Option<(ArgT, ValueT)> {
        self.intervals
            .range((Unbounded, Included(arg)))
            .next_back()
            .map(|x| (*x.0, *x.1))
    }

    fn upper_bound(&self, arg: ArgT) -> Option<(ArgT, ValueT)> {
        self.intervals
            .range((Included(arg), Unbounded))
            .next()
            .map(|x| (*x.0, *x.1))
    }

    fn combine_data_points(&self, other: &Self) -> BTreeSet<ArgT> {
        let args_self = self.intervals.keys();
        let args_other = other.intervals.keys();
        args_self.chain(args_other).copied().collect()
    }

    pub fn intersect(&self, other: &Self) -> Option<(ArgT, ValueT)> {
        // Functions might not intersect. Outside algorithms scope.
        if self.min_value > other.min_value && self.max_value > other.max_value {
            return None;
        }
        if self.min_value < other.min_value && self.max_value < other.max_value {
            return None;
        }

        let (f_smaller, f_greater) = if self.min_value < other.min_value {
            (self, other)
        } else {
            (other, self)
        };

        let mut min = min(f_smaller.min_arg, f_greater.min_arg);
        let mut max = max(f_smaller.max_arg, f_greater.max_arg);

        let eps = ArgT::new(1e-6);
        while max - min > eps {
            let mid = (min + max) / 2.;
            let smaller_value = f_smaller.value(mid);
            let greater_value = f_greater.value(mid);
            if smaller_value < greater_value {
                min = mid;
            } else {
                max = mid;
            }
        }
        Some((min, f_smaller.value(min)))
    }

    pub fn intervals(&self) -> Vec<(ArgT, ValueT)> {
        let mut res = Vec::from_iter(self.intervals.clone().into_iter());
        res.sort_unstable_by_key(|x| x.0);
        res
    }

    pub fn min_arg(&self) -> ArgT {
        self.min_arg
    }

    pub fn min_value(&self) -> ValueT {
        self.min_value
    }

    pub fn max_arg(&self) -> ArgT {
        self.max_arg
    }

    pub fn max_value(&self) -> ValueT {
        self.max_value
    }
}

impl FunctionAbstract for Function {
    fn value(&self, arg: ArgT) -> ValueT {
        match (self.lower_bound(arg), self.upper_bound(arg)) {
            (Some((lower_arg, lower_val)), Some((upper_arg, upper_val))) => {
                if lower_arg == upper_arg {
                    lower_val
                } else {
                    let arg_diff = (arg - lower_arg).float();
                    let arg_range = (upper_arg - lower_arg).float();
                    let val_diff = (upper_val - lower_val).float();
                    let change = val_diff * (arg_diff / arg_range);
                    lower_val + ValueT::new(change)
                }
            }
            (Some((_, lower_val)), None) => lower_val,
            (None, Some((_, upper_val))) => upper_val,
            (None, None) => unreachable!(),
        }
    }

    fn add_value(&mut self, value: ValueT) -> &mut Self {
        self.min_value += value;
        self.max_value += value;
        self.intervals = self
            .intervals
            .iter()
            .map(|(k, v)| (*k, *v + value))
            .collect();
        self
    }

    fn substract_value(&mut self, value: ValueT) -> &mut Self {
        self.add_value(-value)
    }

    fn add_function(&mut self, function: &Self) -> &mut Self {
        let args_combined = Function::combine_data_points(self, function);
        let intervals: BTreeMap<ArgT, ValueT> = args_combined
            .into_iter()
            .map(|arg| (arg, self.value(arg) + function.value(arg)))
            .collect();

        self.min_arg = min(self.min_arg, function.min_arg);
        self.min_value += function.min_value;

        self.max_arg = max(self.max_arg, function.max_arg);
        self.max_value += function.max_value;

        self.intervals = intervals;

        self
    }

    fn substract_function(&mut self, function: &Self) -> &mut Self {
        let args_combined = Function::combine_data_points(self, function);
        let intervals: BTreeMap<ArgT, ValueT> = args_combined
            .into_iter()
            .map(|arg| (arg, self.value(arg) - function.value(arg)))
            .collect();

        self.min_arg = min(self.min_arg, function.min_arg);
        self.min_value -= function.min_value;

        self.max_arg = max(self.max_arg, function.max_arg);
        self.max_value -= function.max_value;

        self.intervals = intervals;

        self
    }

    fn shift_right(&mut self, shift: ArgT) -> &mut Self {
        self.min_arg += shift;
        self.max_arg += shift;
        self.intervals = self
            .intervals
            .iter()
            .map(|(k, v)| (*k + shift, *v))
            .collect();
        self
    }

    fn shift_left(&mut self, shift: ArgT) -> &mut Self {
        self.shift_right(-shift)
    }
}

impl Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let result = Vec::serialize(&Vec::from_iter(self.intervals.iter()), serializer)?;
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for Function {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values: Vec<(ArgT, ValueT)> = Vec::deserialize(deserializer)?;
        Ok(Function::new(values.into_iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::testing::make_function;
    use crate::util::testing::test_eq_arg;
    use crate::util::testing::test_eq_value;

    #[cfg(test)]
    mod value_access {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = make_function(vec![(1., 3.), (5., 7.)]);
            test_eq_value(fun.value(ArgT::new(3.)), ValueT::new(5.));
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(4.));
        }

        #[test]
        fn basic_2() {
            let fun = make_function(vec![(1., 3.), (2., 7.), (5., 10.)]);
            test_eq_value(fun.value(ArgT::new(1.5)), ValueT::new(5.));
            test_eq_value(fun.value(ArgT::new(4.)), ValueT::new(9.));
        }

        #[test]
        fn basic_3() {
            let fun = make_function(vec![(1., 3.), (9., 7.), (15., 10.)]);
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(3.));
            test_eq_value(fun.value(ArgT::new(9.)), ValueT::new(7.));
            test_eq_value(fun.value(ArgT::new(15.)), ValueT::new(10.));
        }

        #[test]
        fn outside_access_1() {
            let fun = make_function(vec![(1., 3.), (2., 2.)]);
            test_eq_value(fun.value(ArgT::new(0.)), ValueT::new(3.));
            test_eq_value(fun.value(ArgT::new(6.)), ValueT::new(2.));
        }
    }

    #[cfg(test)]
    mod modification {
        use super::*;

        #[test]
        fn add_value_1() {
            let mut fun = make_function(vec![(1., 4.), (2., 6.)]);
            fun.add_value(ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(8.));
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(10.));
        }

        #[test]
        fn add_value_2() {
            let mut fun = make_function(vec![(4., 4.), (8., 6.), (10., 8.)]);
            fun.add_value(ValueT::new(-2.));
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(2.));
            test_eq_value(fun.value(ArgT::new(6.)), ValueT::new(3.));
            test_eq_value(fun.value(ArgT::new(9.)), ValueT::new(5.));
            test_eq_value(fun.value(ArgT::new(14.)), ValueT::new(6.));
        }

        #[test]
        fn substract_value_1() {
            let mut fun = make_function(vec![(1., 4.), (2., 6.)]);
            fun.substract_value(ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(-1.)), ValueT::new(0.));
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(0.));
            test_eq_value(fun.value(ArgT::new(3.)), ValueT::new(2.));
        }

        #[test]
        fn add_function_1() {
            let mut fun = make_function(vec![(1., 4.), (3., 6.)]);
            let other = make_function(vec![(1., 5.), (3., 7.)]);
            fun.add_function(&other);
            test_eq_value(fun.value(ArgT::new(-1.)), ValueT::new(9.));
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(11.));
            test_eq_value(fun.value(ArgT::new(4.)), ValueT::new(13.));
        }

        #[test]
        fn add_function_2() {
            let mut fun = make_function(vec![(1., 4.), (3., 6.), (9., 9.)]);
            let other = make_function(vec![(1., 5.), (6., 7.), (9., 10.)]);
            fun.add_function(&other);
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(10.4));
            test_eq_value(fun.value(ArgT::new(4.)), ValueT::new(12.7));
            test_eq_value(fun.value(ArgT::new(8.)), ValueT::new(17.5));
        }

        #[test]
        fn add_function_3() {
            let mut fun = make_function(vec![(3., 4.), (5., 6.)]);
            let other = make_function(vec![(1., 5.), (11., 7.)]);
            fun.add_function(&other);
            test_eq_value(fun.value(ArgT::new(-1.)), ValueT::new(9.));
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(9.));
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(9.2));
            test_eq_value(fun.value(ArgT::new(4.)), ValueT::new(10.6));
            test_eq_value(fun.value(ArgT::new(8.)), ValueT::new(12.4));
        }

        #[test]
        fn shift_left_1() {
            let mut fun = make_function(vec![(3., 4.), (5., 6.)]);
            fun.shift_left(ArgT::new(2.));
            test_eq_value(fun.value(ArgT::new(-1.)), ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(1.)), ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(2.)), ValueT::new(5.));
            test_eq_value(fun.value(ArgT::new(3.)), ValueT::new(6.));
            test_eq_value(fun.value(ArgT::new(5.)), ValueT::new(6.));
        }

        #[test]
        fn shift_right_1() {
            let mut fun = make_function(vec![(3., 4.), (5., 6.)]);
            fun.shift_right(ArgT::new(2.));
            test_eq_value(fun.value(ArgT::new(3.)), ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(5.)), ValueT::new(4.));
            test_eq_value(fun.value(ArgT::new(6.)), ValueT::new(5.));
            test_eq_value(fun.value(ArgT::new(7.)), ValueT::new(6.));
            test_eq_value(fun.value(ArgT::new(9.)), ValueT::new(6.));
        }
    }

    #[cfg(test)]
    mod intersection {
        use super::*;

        #[test]
        fn basic_1() {
            let fun_1 = make_function(vec![(3., 4.), (5., 6.)]);
            let fun_2 = make_function(vec![(3., 6.), (5., 4.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(4.));
            test_eq_value(val, ValueT::new(5.));
        }

        #[test]
        fn basic_2() {
            let fun_1 = make_function(vec![(3., 4.), (5., 6.), (7., 10.)]);
            let fun_2 = make_function(vec![(3., 11.), (5., 8.), (7., 4.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(5.5));
            test_eq_value(val, ValueT::new(7.));
        }

        #[test]
        fn basic_3() {
            let fun_1 = make_function(vec![
                (0., 8.),
                (2., 7.),
                (4., 4.),
                (7., 3.),
                (8., 1.),
                (10., 0.),
            ]);
            let fun_2 = make_function(vec![
                (0., 0.),
                (2., 2.),
                (4., 3.),
                (7., 4.),
                (8., 5.),
                (10., 8.),
            ]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(5.5));
            test_eq_value(val, ValueT::new(3.5));
        }

        #[test]
        fn unaligned_1() {
            let fun_1 = make_function(vec![(0., 9.), (2., 8.), (6., 2.), (7., 1.)]);
            let fun_2 = make_function(vec![(0., 3.), (3., 4.), (5., 6.), (7., 7.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(4.));
            test_eq_value(val, ValueT::new(5.));
        }

        #[test]
        fn unaligned_2() {
            let fun_1 = make_function(vec![(0., 5.), (3., 2.), (6., 1.), (7., -2.), (8., -3.)]);
            let fun_2 = make_function(vec![(-2., 0.), (1., 2.), (3., 5.), (6., 6.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(1.8));
            test_eq_value(val, ValueT::new(3.2));
        }

        #[test]
        fn node_1() {
            let fun_1 = make_function(vec![
                (0., 5.),
                (2., 3.),
                (3., 2.),
                (6., 1.),
                (7., -2.),
                (8., -3.),
            ]);
            let fun_2 = make_function(vec![(-2., 0.), (1., 2.), (2., 3.), (3., 5.), (6., 6.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(2.));
            test_eq_value(val, ValueT::new(3.));
        }

        #[test]
        fn empty_1() {
            let fun_1 = make_function(vec![(0., 4.)]);
            let fun_2 = make_function(vec![(1., 5.)]);
            assert_eq!(fun_1.intersect(&fun_2), None);
        }

        #[test]
        fn empty_2() {
            let fun_1 = make_function(vec![(0., 4.)]);
            let fun_2 = make_function(vec![(1., 4.)]);
            let (_, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_value(val, ValueT::new(4.));
        }

        #[test]
        fn outside_1() {
            let fun_1 = make_function(vec![(-1., 5.), (1., 1.), (3., 0.)]);
            let fun_2 = make_function(vec![(2., 2.), (4., 4.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(0.5));
            test_eq_value(val, ValueT::new(2.));
        }

        #[test]
        fn outside_2() {
            let fun_1 = make_function(vec![(3., 1.), (5., -2.), (7., -3.)]);
            let fun_2 = make_function(vec![(0., -2.), (2., 2.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(1.5));
            test_eq_value(val, ValueT::new(1.));
        }

        #[test]
        fn outside_3() {
            let fun_1 = make_function(vec![(0., 0.), (2., 2.), (4., 2.)]);
            let fun_2 = make_function(vec![(2., 4.), (4., 0.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(3.));
            test_eq_value(val, ValueT::new(2.));
        }

        #[test]
        fn outside_4() {
            let fun_1 = make_function(vec![(-1., 5.), (1., 3.), (2., 0.)]);
            let fun_2 = make_function(vec![(0., -2.), (3., -1.), (4., 1.), (6., 2.)]);
            let (arg, val) = fun_1.intersect(&fun_2).unwrap();
            test_eq_arg(arg, ArgT::new(3.5));
            test_eq_value(val, ValueT::new(0.));
        }
    }
}

//! Base representation of mathematical function.


use std::cmp::max;
use std::cmp::min;

trait Function {
    type ArgT;
    type ValueT;

    fn arg_min(&self) -> Self::ArgT;
    fn arg_max(&self) -> Self::ArgT;

    fn value_at(&self, arg: Self::ArgT) -> Self::ValueT;
    fn value_at_interval(&self, arg_min: Self::ArgT, arg_max: Self::ArgT) -> Vec<(Self::ArgT, Self::ArgT, Self::ValueT)>;

    fn add_value(&mut self, val: Self::ValueT);
    fn add_function(&mut self, fun: Self);
    fn shift(&mut self, val: Self::ArgT);
}


struct FunctionVec {
    arg_min: i32,
    values: Vec<i32>
}

impl FunctionVec {
    fn new(arg_min: i32, values: Vec<i32>) -> FunctionVec {
        FunctionVec{arg_min, values}
    }
}

impl Function for FunctionVec {
    type ArgT = i32;
    type ValueT = i32;

    fn arg_min(&self) -> Self::ArgT {
        self.arg_min
    }

    fn arg_max(&self) -> Self::ArgT {
        self.arg_min + (self.values.len() as i32)
    }

    fn value_at(&self, arg: Self::ArgT) -> Self::ValueT {
        if arg < self.arg_min() { self.values[0] }
        else if arg >= self.arg_max() { self.values[self.values.len() - 1] }
        else { self.values[(arg - self.arg_min()) as usize] }
    }

    fn value_at_interval(&self, arg_min: Self::ArgT, arg_max: Self::ArgT) -> Vec<(Self::ArgT, Self::ArgT, Self::ValueT)> {
        let mut res = vec!{};
        let res_min_arg = min(arg_min, self.arg_min());
        let res_max_arg = max(arg_max, self.arg_max());
        
        for i in res_min_arg..res_max_arg {
            res.push((i, i + 1, self.value_at(i)))
        };
        res
    }

    fn add_value(&mut self, val: Self::ValueT) {
        for i in &mut self.values {
            *i += val
        }
    }

    fn add_function(&mut self, fun: Self) {
        let new_min_arg = min(self.arg_min(), fun.arg_min());
        let new_max_arg = max(self.arg_max(), fun.arg_max());
        
        let mut new_values = vec!{};

        for i in new_min_arg..new_max_arg {
            new_values.push(self.value_at(i) + fun.value_at(i))
        }

        self.arg_min = new_min_arg;
        self.values = new_values;
    }
    
    fn shift(&mut self, val: Self::ArgT) {
        self.arg_min += val
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
            let fun = FunctionVec::new(15, vec!{1, 2, 3, 4, 5, 6});
            assert_eq!(fun.arg_min(), 15);
            assert_eq!(fun.arg_max(), 21);
        }
    }

    #[cfg(test)]
    mod value_access {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = FunctionVec::new(0, vec!{1, 2, 3, 4, 5, 6, 7, 8});
            assert_eq!(fun.value_at(2), 3);
            assert_eq!(fun.value_at(4), 5);
        }

        #[test]
        fn basic_2() {
            let fun = FunctionVec::new(-5, vec!{9, 5, 4, 6, 8, 2});
            assert_eq!(fun.value_at(-3), 4);
            assert_eq!(fun.value_at(0), 2);
            assert_eq!(fun.value_at(-4), 5);
        }

        #[test]
        fn outside_access() {
            let fun = FunctionVec::new(7, vec!{1, 5, 4, 2});
            assert_eq!(fun.value_at(6), 1);
            assert_eq!(fun.value_at(54), 2);
        }
    }

    #[cfg(test)]
    mod interval_access {
        use super::*;

        #[test]
        fn basic_1() {
            let fun = FunctionVec::new(2, vec!{1, 2, 3});
            assert_eq!(fun.value_at_interval(2, 5), vec!((2, 3, 1), (3, 4, 2), (4, 5, 3)));
        }
    }

    #[cfg(test)]
    mod modification {
        use super::*;

        #[test]
        fn add_value_1() {
            let mut fun = FunctionVec::new(6, vec!{5, 4, 6, 8, 7});
            fun.add_value(6);
            assert_eq!(fun.values, vec!{11, 10, 12, 14, 13});
        }

        #[test]
        fn add_value_2() {
            let mut fun = FunctionVec::new(-4, vec!{1});
            fun.add_value(-8);
            assert_eq!(fun.values, vec!{-7});
        }

        #[test]
        fn add_function_1() {
            let mut fun_1 = FunctionVec::new(5, vec!{8, 7, 5, 4});
            let fun_2 = FunctionVec::new(5, vec!{1, 2, 5, 7});
            fun_1.add_function(fun_2);
            assert_eq!(fun_1.arg_min, 5);
            assert_eq!(fun_1.values, vec!{9, 9, 10, 11});
        }

        #[test]
        fn add_function_2() {
            let mut fun_1 = FunctionVec::new(2, vec!{2, 3, 4});
            let fun_2 = FunctionVec::new(2, vec!{5, 8});
            fun_1.add_function(fun_2);
            assert_eq!(fun_1.arg_min, 2);
            assert_eq!(fun_1.values, vec!{7, 11, 12});
        }

        #[test]
        fn add_function_3() {
            let mut fun_1 = FunctionVec::new(1, vec!{5, 7, 6});
            let fun_2 = FunctionVec::new(-2, vec!{6, 8, 4, 5, 3, 9, 5, 4});
            fun_1.add_function(fun_2);
            assert_eq!(fun_1.arg_min, -2);
            assert_eq!(fun_1.values, vec!{11, 13, 9, 10, 10, 15, 11, 10});
        }

        #[test]
        fn shift_1() {
            let mut fun_1 = FunctionVec::new(4, vec!{3, 5});
            fun_1.shift(-7);
            assert_eq!(fun_1.arg_min, -3);
        }
    }
}
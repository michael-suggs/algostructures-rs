// use std::collections::HashMap;
// use std::hash::Hash;

// struct Cacher<T, U, V>
// where
//     T: Fn(U) -> V,
//     U: Eq + Hash,
// {
//     calculation: T,
//     value: HashMap<U, V>,
// }

// impl<T, U, V> Cacher<T, U, V>
// where
//     T: Fn(U) -> V,
//     U: Eq + Hash,
// {
//     fn new(calculation: T) -> Cacher<T, U, V> {
//         Cacher {
//             calculation,
//             value: HashMap::new(),
//         }
//     }

//     fn value(&mut self, arg: U) -> &V {
//         match self.value.get(&arg) {
//             Some(v) => &v,
//             None => {
//                 let result: V = (self.calculation)(arg);
//                 self.value.insert(arg, result);
//                 &result
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn call_with_different_values() {
//         let mut c = Cacher::new(|a| a);

//         let v1 = c.value(1);
//         let v2 = c.value(2);

//         assert_eq!(v2, 2);
//     }
// }

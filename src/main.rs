// mod dict;
// use dict::Dict;
extern crate proc_macro_local;
use proc_macro_local::*;

use std::{
    any::Any,
    collections::HashMap,
    hash::Hash,
    ops::{AddAssign, Index, IndexMut},
};

#[derive(Debug)]
pub struct Dict<T> {
    pub(self) hash: HashMap<T, Box<dyn Any>>,
}
impl<T> Dict<T>
where
    T: Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Dict {
            hash: HashMap::new(),
        }
    }
    pub fn insert<U: 'static>(&mut self, key: T, hole: U) {
        self.hash.insert(key, Box::new(hole));
    }

    pub fn get<U: 'static>(&self, key: &T) -> Result<&U, DictError> {
        // match self.hash.get(&key).unwrap().as_ref().downcast_ref::<U>()
        match self.hash.get(&key) {
            Some(boxed_value) => match boxed_value.as_ref().downcast_ref::<U>() {
                Some(value) => Ok(value),
                None => Err(DictError::FailedCasting(
                    std::any::type_name::<U>().to_string(),
                )),
            },
            None => Err(DictError::KeyNotFound),
        }
    }
    pub fn get_mut<U: 'static>(&mut self, key: &T) -> Result<&mut U, DictError> {
        // match self.hash.get(&key).unwrap().as_ref().downcast_ref::<U>()
        match self.hash.get_mut(&key) {
            Some(boxed_value) => match boxed_value.as_mut().downcast_mut::<U>() {
                Some(value) => Ok(value),
                None => Err(DictError::FailedCasting(
                    std::any::type_name::<U>().to_string(),
                )),
            },
            None => Err(DictError::KeyNotFound),
        }
    }
}
#[derive(Debug)]
pub enum DictError {
    KeyNotFound,
    FailedCasting(String),
}

impl<T> Index<T> for Dict<T>
where
    T: Eq + PartialEq + Hash,
{
    type Output = Box<dyn Any>;

    fn index(&self, index: T) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

#[test]
fn testing_index() {
    let mut daict: Dict<&str> = Dict::new();
    daict.insert("key", 0.);
    println!("{:?}", daict);
    println!("{:?}", daict["key"]);
    // daict["lol"] = Box::new(45.0);
    // println!("{:?}", daict);
}

// // currently impossible as we can't know the wanted ref type (&T) wanted
// impl<T> IndexMut<T> for Dict<T>
// where
//     T: Eq + PartialEq + Hash + Clone,
//     // U: Default,
// {
//     fn index_mut(&mut self, index: T) -> &mut Self::Output {
//         if !self.hash.contains_key(&index) {
//             self.insert(index.clone(), Box::new(None::<Box<dyn Any>>));
//         }
//         self.get_mut(&index).unwrap()
//     }
// }

// impl<T, U> AddAssign<U> for Dict<T>
// where
//     T: Eq + PartialEq + Hash,
//     U: Default,
// {
//     fn add_assign(&mut self, rhs: U) {
//         if !self.hash.contains_key(&rhs) {
//             self.insert(rhs.clone(), Box::new(rhs));
//         }
//         self.get_mut(&rhs).unwrap()
//     }
// }
fn main() {
    println!("Hello, world!");
    // let mut daict: Dict<&str> = Dict::new();
    // println!("{:?}", daict);
    // daict["lol"] = Box::new(45.0);
    // println!("{:?}", daict);
}

//! WIP: not fully functionnal/practical
use std::{
    any::Any,
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Dict<T> {
    pub(self) hash: HashMap<T, Box<dyn Any>>,
}
/// WIP: not fully functionnal/practical
/// there is a way of getting a type name with `std::any::type_name::<T>()` which returns `"T" as &str`
/// with a macro we can get the reversed processus by parsing the `"T"` inside `downcast_ref::<T>()`
/// allowing a behavior such as `let value = dict.get!(my_key).unwrap();`, which could be simplified as
/// `let value = dict[my_key]`, instead of `let value: &T = dict.get::<T>(my_key).unwrap();`
/// `.unwrap()` is needed as it is the responsibility of the coder to handle missbehavior of the dict usage
/// # Example of current usage
/// ```rust
/// # use cats_utils::dict::Dict;
/// let mut my_dict = Dict::new();
/// my_dict.insert("key", "value");
/// println!("{}", my_dict.get::<&str>("key").unwrap());
/// ```
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
                None => Err(DictError::FailedCasting),
            },
            None => Err(DictError::KeyNotFound),
        }
    }
    pub fn get_mut<U: 'static>(&mut self, key: &T) -> Result<&mut U, DictError> {
        // match self.hash.get(&key).unwrap().as_ref().downcast_ref::<U>()
        match self.hash.get_mut(&key) {
            Some(boxed_value) => match boxed_value.as_mut().downcast_mut::<U>() {
                Some(value) => Ok(value),
                None => Err(DictError::FailedCasting),
            },
            None => Err(DictError::KeyNotFound),
        }
    }
}
#[derive(Debug)]
pub enum DictError {
    KeyNotFound,
    FailedCasting,
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

impl<T> IndexMut<T> for Dict<T>
where
    T: Eq + PartialEq + Hash + Clone,
    // U: Default,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        match self.hash.get(&index) {
            // Ok(_) => (),
            // Err(_) => {
            Some(_) => (),
            None => {
                self.insert(index.clone(), Box::new(None::<Box<dyn Any>>));
            }
        }
        self.get_mut(&index).unwrap()
    }
}
// #[test]
// fn testing_dict() {
//     println!("Hello, world!");
//     let mut daict: Dict<&str> = Dict::new();
//     println!("{:?}", daict);
//     daict["lol"] = Box::new(45.0);
//     println!("{:?}", daict);
// }

// impl<T: 'static> Index<T> for Dict<T>
// where
//     T: Eq + PartialEq + Hash,
// {
//     type Output = Result<Box<dyn Any>, DictError>;

//     fn index(&self, index: T) -> &Self::Output {
//         self.get(index).unwrap()
//     }
// }

// struct A;
// struct B {
//     f: usize,
// }

// struct Saze<T, Marker> {
//     first: T,
//     second: Marker,
// }

// trait MarkerTrait: Sized {
//     fn mark<T>(&self, para: &Saze<T, Self>);
// }

// impl MarkerTrait for A {
//     fn mark<T>(&self, para: &Saze<T, Self>) {
//         todo!()
//     }
// }
// impl MarkerTrait for B {
//     fn mark<T>(&self, para: &Saze<T, Self>) {
//         todo!()
//     }
// }
// impl<T> Saze<T, A> {
//     fn new() {}
// }
// impl<T> Saze<T, B> {
//     fn new() {}
// }

// impl<T, Marker> Saze<T, Marker>
// where
//     T: Sized,
//     Marker: MarkerTrait,
// {
//     fn working(&self) {
//         self.second.mark(self)
//     }
// }

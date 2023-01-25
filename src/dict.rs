//! WIP: not fully functionnal/practical
use std::{any::Any, collections::HashMap, hash::Hash};

pub struct Dict<T> {
    pub(self) inner: HashMap<T, Box<dyn Any>>,
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
            inner: HashMap::new(),
        }
    }
    pub fn insert<U: 'static>(&mut self, key: T, hole: U) {
        self.inner.insert(key, Box::new(hole));
    }

    pub fn get<U: 'static>(&self, key: T) -> Result<&U, DictError> {
        // match self.inner.get(&key).unwrap().as_ref().downcast_ref::<U>()
        match self.inner.get(&key) {
            Some(boxed_value) => match boxed_value.as_ref().downcast_ref::<U>() {
                Some(value) => Ok(value),
                None => Err(DictError::FailedCasting),
            },
            None => Err(DictError::KeyNotFound),
        }
    }
    pub fn get_mut<U: 'static>(&mut self, key: T) -> Result<&mut U, DictError> {
        // match self.inner.get(&key).unwrap().as_ref().downcast_ref::<U>()
        match self.inner.get_mut(&key) {
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

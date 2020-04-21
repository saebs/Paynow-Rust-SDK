use std::collections::HashMap;
// use std::fmt::{Debug, Display};
// use std::hash::Hash;
// For use later when creating generics
//use std::iter::Iterator;

pub trait Transact {
/// creates a new instance of a Transaction
    fn new() -> Self;
    /// format transaction for client
    fn init(&self) -> String;
    //fn load<T>(&mut self, data: Box<T>) {}
    
    fn load(&mut self, _map: &HashMap<String, String>) {
    }
    // TODO
    // implement generic methods that accepts standard key value pair collection
    
/*
    fn load<K: Debug + Eq + Hash + Display, V: Debug + Display>(&mut self, map: &HashMap<K, V>) {
        for (k, v) in map.iter() {
            println!("{:?}: {:?}", k, v);
        }
    }

    */
    
}

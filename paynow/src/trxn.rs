/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

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

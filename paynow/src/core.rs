/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 
//  Paynow Rust SDK

use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use std::fmt::{Write};
use crate::fields::*;

/// Paynow type for interacting with paynow 
#[derive( Debug, PartialEq)]
pub struct Paynow {
    pub integration_id: &'static str, 
    pub integration_key: &'static str,
    pub returnurl: &'static str,
    pub resulturl: &'static str, 
    pub tokenize: bool,
}


impl Paynow {
    /// Creates an empty instance for Paynow Type 
    pub fn new() -> Paynow {
        // If merchant is registered to use token it needs to be set to True later 
        Paynow {integration_id: "", integration_key: "", returnurl: "", resulturl: "", tokenize: false}
    }

    /// Create Paynow instance from key - value pairs
    // Data sources could be e.g. HashMap, text file maybe? 
    // or JSON i dont know. 
    // To KISS it we will use a hashmap 
    /// NB: Not production ready,
    pub fn from<T, V>(source: HashMap<T,V>) -> Paynow {
        // TODO Fix this nonsense before  putting to production` 
        let paynow = Paynow {integration_id: "kung", integration_key: "foo", returnurl: "", resulturl: "", tokenize: false};
        paynow
    }


    }


/// Helper for composing transactions before posting to Paynow
#[derive(Clone, Debug)]
pub struct Payment {
    pub reference: &'static str, // unique identifier for transaction
    pub items: HashMap<&'static str, usize>,  // Dictionary of items in shopping cart description and amount
    pub auth_email: &'static str, // Users email address
    pub additionalinfo: &'static str,
    pub amount: isize,
}


//Personal notes
// Get data from paynow, analysise and extract required fields for specific transaction
impl Payment {
    pub fn new() -> Payment {
        Payment {reference: "", items: HashMap::new(), auth_email: "", additionalinfo: "", amount: 0}
    }
/// Add item to trolley , Muno tinoti trolley, 'cart' kuti kudii?
// We want to use cents for now till i figure out best data type to use.
// Paynow recommends max of two decimal places for amounts, so maybe we can work around this
// from first principles
    pub fn add(&mut self, item: &'static str, price: usize) {
        self.items.insert(item, price);

    }

    // pub fn add(&mut self, item: &'static str, amount: isize) {
    //     //TODO iterate cart and get total amount
    //     self.items.insert(item.to_owned(), amount);
 
    // }
    // remove from cart
    
    // Payment Total
    
    // need to get polls status.....
}
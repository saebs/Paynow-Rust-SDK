//! Core  types for interfacing with Paynow API

/*
Copyright (C) 2020 by Saziwe PBC

Permission to use, copy, modify, and/or distribute this software for any purpose with
or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE
INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE
FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

// FOR FUTURE FEATURES
// use hyper::Request;
// use reqwest::Client;
// use serde::{Deserialize, Serialize};

use crate::transactions::Transaction;
// use crate::types::{PaymentMethod,Status};
use crate::utils;
use std::collections::HashMap;
use std::num::ParseFloatError;

type InitRequest = String;

/// The Paynow type that models the official API spec / Abstract Type
#[derive(Default, Debug, PartialEq)]
pub struct Paynow {
    integration_id: String,
    integration_key: String,
    returnurl: String,
    resulturl: String,
    tokenize: bool,
}

/*
StatusResponse	PollTransaction(String url)
StatusResponse	ProcessStatusUpdate(String response)
StatusResponse	ProcessStatusUpdate(Dictionary<String, String> response)
*/
impl Paynow {
    /// Creates an new instance for the Paynow Type
    /// Assumes the primitive and custom type defaults
    pub fn new() -> Self {
        // If merchant is registered to use token it needs to be set to True later
        Paynow {
            ..Default::default()
        }
    }

    /// Creates a Paynow instance from a collection of key - value pairs
    /// This could be from any preformatted data source
    pub fn from(_form: HashMap<&str, &str>) {
        // what now
    }

    // Setters

    pub fn set_integration_id(&mut self, integration_id: &str) {
        self.integration_id = integration_id.to_string();
    }
    pub fn set_integration_key(&mut self, integration_key: &str) {
        self.integration_key = integration_key.to_string();
    }

    pub fn set_resulturl(&mut self, resulturl: &str) {
        self.resulturl = resulturl.to_string()
    }

    pub fn set_returnurl(&mut self, returnurl: &str) {
        self.returnurl = returnurl.to_string()
    }

    pub fn tokenize(&mut self, tokenize: bool) {
        self.tokenize = tokenize;
    }

    /// Create New Instance of Payment handler / helper
    pub fn create_payment(&mut self, reference: &str, auth_email: &str) -> Payment {
        let mut trxn = Transaction::new();
        trxn.set_reference(reference);
        trxn.set_authemail(auth_email);
        Payment {
            transaction: trxn,
            ..Default::default()
        }
    }

    /// Purpose: to send or initialise a payment request to Paynow API
    pub fn send(&self, _payment: Payment) -> InitRequest {
        //initiate new clients and send req
        // set Transaction type requied to txn
        unimplemented!()
    }

    /// Change payment mode/type to mobile,
    /// works something like paynow.mobile("0777000000", "onemoney").send(payment)
    pub fn mobile(&self, _phone: &str, _method: &str) {
        // auth email is mandatory
        unimplemented!()
    }
}

/// Payments Handler Type
// handles a shopping trolley (we speak British English around here!),
// Including other transactional requirements
#[derive(Default, Debug, PartialEq)]
pub struct Payment {
    // basket or trolley goods  'description and amount
    items: HashMap<&'static str, usize>,
    transaction: Transaction,
}

impl Payment {
    pub fn new() -> Self {
        let payment = Default::default();
        payment
    }

    /// Add item to trolley
    // Paynow recommends max of two decimal places for amounts
    pub fn add(&mut self, item: &'static str, price: &str) -> Result<(), ParseFloatError> {
        // we want to store all amounts in memory in cents
        //
        // parse to cents
        self.items.insert(item, utils::to_cents(price).unwrap());
        Ok(())
    }

    /// remove item from trolley or basket
    pub fn remove(&mut self, item: &'static str) {
        self.items.remove(item);
    }

    /// Shopping total
    #[allow(dead_code)]
    fn total(&mut self) -> usize {
        let mut amt = 0;
        for i in self.items.values() {
            amt += i;
        }
        amt
    }
    // need to get polls status.....
}

#[cfg(test)]
mod tests {
    #[test]
    fn describe_test() {
        // Prove that 1 ->  ~2
        assert_eq!(1, 1);
    }
}

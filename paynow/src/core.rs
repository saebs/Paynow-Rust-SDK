/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/
//  Rust SDK for Paynow Zimbabwe's API
/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/


// use hyper::Request;
// use reqwest::Client;
use serde::{Deserialize, Serialize};

// use crate::responses::*;
use crate::transactions::Transaction;
// use crate::types::{PaymentMethod,Status};
// use crate::utils;
use std::collections::HashMap;
use std::num::ParseFloatError;

/// The main type for interacting with Paynow
#[derive(Debug, PartialEq)]
pub struct Paynow {
    //TODO make idiomatic
    // write setters and getters for these parameters and hide em
    integration_id: &'static str,
    integration_key: &'static str,
    returnurl: &'static str,
    resulturl: &'static str,
    tokenize: bool,
}

/*
CreatePayment(String reference, Dictionary<String, Decimal> values = null, String authEmail = )
StatusResponse	PollTransaction(String url)
StatusResponse	ProcessStatusUpdate(String response)
StatusResponse	ProcessStatusUpdate(Dictionary<String, String> response)
InitResponse	Send(Payment payment)
InitResponse	SendMobile(Payment payment, String phone, MobileMoneyMethod method = Ecocash)
*/
#[allow(dead_code)]
impl Paynow {
    /// Creates an empty instance for Paynow Type
    pub fn new() -> Self {
        // If merchant is registered to use token it needs to be set to True later
        Paynow {
            integration_id: "",
            integration_key: "",
            returnurl: "",
            resulturl: "",
            tokenize: false,
        }
    }

    /// Create a Payment
    pub fn create_payment(&mut self, reference: &'static str, auth_email: &'static str) -> Payment {
        let items = HashMap::new();
    }

   
    // TODO , iwrite send or init transaction functionality
    // Purpose: to send or init regular payment request
    // Sign: send(payment) -> InitResponse
    /// Request to initialise a transaction
    pub fn send(&self, payment: Payment) -> impl Serialize + Deserialize {
        //initiate new clients and send req

        // set Transaction type requied to txn

        unimplemented!()
    }

    //TODO write send mobile method
    // Purpose : to send or initiate an express checkout / mobile payment
    // mo
    // SendMobile(payment,phone, method) -> InitResponse
    pub fn mobile(&self, phone: &'static str, method: &'static str) {
    // put constraints , auth email mandatory
    // just adds mobile payment required info if not already loaded 
    // then calls send or we just chain the idiotic way
        unimplemented!()
    }

}

/// Helper for composing transactions before posting to Paynow
#[derive(Debug, PartialEq)]
pub struct Payment {
    pub items: HashMap<&'static str, usize>, // Dictionary of items in shopping cart description and amount
    pub amount: usize,
    transaction: Transaction,
}

type Money = u64;
//Personal notes
// Get data from paynow, analysise and extract required fields for specific transaction
impl Payment {
    pub fn new(reference: &'static str, auth_email: &'static str, additionalinfo: &'static str, amount: &'static str) -> Self {
    }

    /// Payment reference setter
    pub fn set_reference(&mut self, reference: &'static str) {
        self.reference = reference;
    }
    /// Authentication email setter
    pub fn set_authemail(&mut self, auth_email: &'static str) {
        self.auth_email = auth_email;
    }
    
    /// Add item to trolley 
    // Paynow recommends max of two decimal places for amounts
    pub fn add(&mut self, item: &'static str, price: &str) -> Result<(), ParseFloatError> {
        // we want to store total amount in cents
        // parse to cents
        self.items.insert(item, (price * 100f64) as u64);
        Ok(())
    }

    /// remove item from trolley or basket
    pub fn remove(&mut self, item: &'static str) {
        self.items.remove(item);
    }

    /// Payment Total
    /// Should used to get shopping total amount , ie update payment amount
    pub fn sum(&mut self) -> usize {
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
        assert_eq!(1 , 1);
    }
}


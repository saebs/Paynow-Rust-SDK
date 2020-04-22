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
// use crate::transactions::*;
// use crate::types::{PaymentMethod,Status};
// use crate::utils;
use std::collections::HashMap;
use std::num::ParseFloatError;

/// The main Model type for interacting with Paynow
#[derive(Debug, PartialEq)]
pub struct Paynow {
    //TODO make idiomatic
    // write setters and getters for these parameters and hide em
    pub integration_id: &'static str,
    pub integration_key: &'static str,
    pub returnurl: &'static str,
    pub resulturl: &'static str,
    pub tokenize: bool,
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
        Payment {
            reference,
            items,
            auth_email,
            additionalinfo: "",
            amount: 0usize,
            requests: "foo".to_owned(),
        }
    }

    /// Create Paynow instance from key - value pairs
    // Data sources could be e.g. HashMap, text file maybe?
    // or JSON i dont know.
    // To KISS it we will use a hashmap
    /// NB: Not production ready,
    #[allow(dead_code)]
    pub fn from<T>(data_src: T) -> Self {
        // supposed to parse json, xml or csv
       unimplemented!() 
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

    pub fn sendmobile(&self, payment: Payment, phone: &'static str, method: &'static str) {
    // just adds mobile payment required info if not already loaded 
        unimplemented!()
    }

    //TODO write send mobile method
    // Purpose : to send or initiate an express checkout / mobile payment
    // mo
    // SendMobile(payment,phone, method) -> InitResponse
}

/// Helper for composing transactions before posting to Paynow
#[derive(Debug, PartialEq)]
pub struct Payment {
    pub reference: &'static str, // unique identifier for transaction
    pub items: HashMap<&'static str, usize>, // Dictionary of items in shopping cart description and amount
    pub auth_email: &'static str,            // Users email address
    pub additionalinfo: &'static str,
    pub amount: usize,
    pub requests: String,
}

//Personal notes
// Get data from paynow, analysise and extract required fields for specific transaction
impl Payment {
    pub fn new() -> Self {
        Payment {
            reference: "",
            items: HashMap::new(),
            auth_email: "",
            additionalinfo: "",
            amount: 0usize,
            requests: String::from("nothing"),
        }
    }

    /// Payment reference setter
    pub fn set_reference(&mut self, reference: &'static str) {
        self.reference = reference;
    }
    /// Authentication email setter
    pub fn set_authemail(&mut self, auth_email: &'static str) {
        self.auth_email = auth_email;
    }
    /// Add item to trolley ehe
    // Paynow recommends max of two decimal places for amounts
    pub fn add(&mut self, item: &'static str, price: &str) -> Result<(), ParseFloatError> {
        let price = price.parse::<f64>()?;
        // we want to store total amount in cents
        self.items.insert(item, (price * 100f64) as usize);
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


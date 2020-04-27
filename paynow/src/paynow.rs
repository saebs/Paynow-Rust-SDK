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
use crate::utils;
use std::collections::HashMap;
use std::num::ParseFloatError;

/// The main type for interacting with Paynow
#[derive(Default, Debug, PartialEq)]
pub struct Paynow {
    //TODO make idiomatic
    // write setters and getters for these parameters and hide em
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
InitResponse	Send(Payment payment)
InitResponse	SendMobile(Payment payment, String phone, MobileMoneyMethod method = Ecocash)
*/
impl Paynow {
    /// Creates an empty instance for Paynow Type
    pub fn new() -> Self {
        // If merchant is registered to use token it needs to be set to True later
        Paynow {
            ..Default::default()
        }
    }

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

    pub fn set_resulturl(&mut self, resulturl: &str){
        self.resulturl = resulturl.to_string()
    }

    pub fn set_returnurl(&mut self, returnurl: &str){
        self.returnurl = returnurl.to_string()
    }

    pub fn tokenize(&mut self, tokenize: bool) {
        self.tokenize = tokenize; 
    }



    /// Create a Payment
    pub fn create_payment(&mut self, reference: &str, auth_email: &str) -> Payment {
        let mut trxn = Transaction::new();
        trxn.set_reference(reference);
        trxn.set_authemail(auth_email);
        Payment{transaction: trxn, ..Default::default()}

    }

   
    // Purpose: to send or init regular payment request
    // Sign: send(payment) -> InitResponse
    /// Request to initialise a transaction
    pub fn send(&self, _payment: Payment) -> impl Serialize + Deserialize {
        //initiate new clients and send req
        // set Transaction type requied to txn

        unimplemented!()
    }

    //TODO write send mobile method
    // Purpose : to send or initiate an express checkout / mobile payment
    // mo
    // SendMobile(payment,phone, method) -> InitResponse
    pub fn mobile(&self, _phone: &str, _method: &str) {
    // put constraints , auth email mandatory
    // just adds mobile payment required info if not already loaded 
    // then calls send or we just chain the idiomatic way
        unimplemented!()
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct Payment {
    items: HashMap<&'static str, usize>, // Dictionary of items in shopping cart description and amount
    amount: usize,
    transaction: Transaction,
}

//Personal notes
// Get data from paynow, analysise and extract required fields for specific transaction
impl Payment {
    pub fn new()  -> Self {
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

    /// Payment Total
    /// Should used to get shopping total amount , ie update payment amount
    #[allow(dead_code)]
    fn sum(&mut self) -> usize {
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


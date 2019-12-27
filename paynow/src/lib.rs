/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 

// Paynow Rust 

/// Core Paynow functionality 
/// like Creating Payments, Handling, Sending and Responses
pub mod core;
/// Paynow Transaction & Response fields , API endpoints and common defaults 
pub mod fields;

/// General utilities for e.g. hashing , Data Collection , Sanitising and non core stuff
pub mod utils;

pub mod responses;

// use pn::{Payment, Paynow};
// client here.....


#[cfg(test)]
mod tests {
use crate::core::{Paynow, Payment};
use std::collections::HashMap;
use crate::utils::*;
    #[test]
    fn creates_paynow_instance() {
        let paynow: Paynow = Paynow::new();   
        let paylater = Paynow {integration_id: "", integration_key: "", returnurl: "", resulturl: "", tokenize: false};
        assert_eq!(format!("{:?}",paynow), format!("{:?}",paylater));
    }

    #[test]
    fn creates_new_empty_payment() {
        let gimme_my_money: Payment = Payment::new();
        let nah =   Payment {
        reference: "", // unique identifier for transaction
        items: HashMap::new(),  // Dictionary of items in shopping cart description and amount
        auth_email: "", // Users email address
        additionalinfo: "",
        amount: 0,
        };

        assert_eq!(format!("{:?}", gimme_my_money), format!("{:?}", nah));
    }

    #[test]
    fn adds_to_basket() {
        let mut tsaona: HashMap<&'static str, usize> = HashMap::new();
        tsaona.insert("chingwa", 1500); // ma Cents usatya!!
        let mut katsapo = Payment::new();
        katsapo.add("chingwa", 1500);
        assert_eq!(tsaona, katsapo.items);
        
    }

    #[test]
    fn remove_from_shopping_basket() {
        let mut katsapo = Payment::new();
        katsapo.add("scud", 700);
        katsapo.remove("scud");
        assert_eq!(katsapo.items.is_empty(), true);

    } 

    #[test]
    fn creates_payement_obj() {

        let mut imbadalo = Paynow::new();
        let mut payment = Payment {
            reference: "000", 
            items: HashMap::new(),  
            auth_email: "your@email.com", 
            additionalinfo: "",
            amount: 0usize,
        };


        assert_eq!(imbadalo.create_payment("000", "your@email.com"),payment );
        // let just test the sum method here
        assert_eq!(0, payment.sum())
    }

    #[test]
    fn hash_util() {
        let message = "1201TEST REF99.99A test ticket transactionhttp://www.google.com/search?q=returnurlhttp://www.google.com/search?q=resulturlMessage";
        assert_eq!(hash_make(message, "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977"), "2A033FC38798D913D42ECB786B9B19645ADEDBDE788862032F1BD82CF3B92DEF84F316385D5B40DBB35F1A4FD7D5BFE73835174136463CDD48C9366B0749C689")

    }
}

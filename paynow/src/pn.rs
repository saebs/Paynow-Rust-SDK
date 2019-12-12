use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::constants::*;
/*Statuses
The following are other possible status settings, 
these will be sent to the merchant site if they change in Paynow or if the merchant polls Paynow for the current status:
*/

/// Status enum to represent Response status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    /// Initiating Transaction status posted to Paynow by Merchant Client
    Message,

    /// If the initiate transaction request is successful 
    Ook, // Ok reserved in rust so Ook will do 

    /// If the initiate transaction request failed 
    Error, 

    /// Paid  Transaction paid successfully, the merchant will receive the funds at next settlement.
    Paid,

    /// Awaiting Delivery Transaction paid successfully, but is sitting in suspense waiting on the merchant to confirm 
    /// delivery of the goods.
    AwaitingDelivery,

    /// Delivered  The user or merchant has acknowledged delivery of the goods but the funds are still sitting
    /// in suspense awaiting the 24 hour confirmation window to close.
    Delivered,

    /// Created Transaction has been created in Paynow, but has not yet been paid by the customer.
    Created,

    /// Sent Transaction has been created in Paynow and an up stream system, 
    /// the customer has been referred to that upstream system but has not yet made payment.
    Sent,

    /// Cancelled The transaction has been cancelled in Paynow and may not be resumed and needs to be recreated.
    Cancelled,

    /// Disputed Transaction has been disputed by the Customer and funds are being held in suspense until 
    /// the dispute has been resolved.
    Disputed,

    /// Refunded Funds were refunded back to the customer.
    Refunded,

    ///Pending 3DS , means card holder is required to complete 3ds Secure payments challenge 
    /// if using VISA / Mastercard
    Pending3ds,
}

/// Passenger Types for Passenger Ticket Transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Passenger {
    /// ADT  Adult	
    Adt,
    /// CNN Child
    Cnn,
    /// INF Infant
    Inf,
    /// YTH youth
    Yth,
    /// STU Student
    Stu,
    /// SCR Senior Citizen
    Scr,
    /// MIL Military
    Mil,
}

/// Mobile Money Payment Methods
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    //Mobile money , String is buyer's phone number
    Ecocash,     
    OneMoney,
    Telecash,
    // Visa / MasterCard
    Vmc,
    All,
}




// Credit / Debit card information 
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    // Numeric	The Visa/Mastercard PAN
    cardnumber: usize,
    // Name printed on front of card
    cardname: String,
    // Numeric	3 or 4 digits from rear of card
    cardcvv: u16,
	// Numeric	6 digit card expiry date (MMYYYY) e.g. 052018
    cardexpiry: u32,
    // String	Customer’s billing address
    billingline1: String,
    // String	Not required but will assist with fraud detection
    billingline2: String,
    // String	Customer’s billing address city
    billingcity: String,
    // String	Not required but will assist with fraud detection
    billingprovince: String,
    // String	Customer’s billing address country
    billingcountry:  String,
}

/// Paynow type for interacting with paynow api
#[derive( Debug, Copy, Serialize, Deserialize, PartialEq)]
pub struct Paynow {
    integration_id: &'static str, 
    integration_key: &'static str,
    /* Merchant's endpoints. */
    returnurl: &'static str,
    resulturl: &'static str, 

    tokenize: bool,
}


impl Paynow {
    // Constructor
    pub fn new(integration_id: &'static str, integration_key: &'static str) -> Result<Paynow, ()> {
        // return url and result are optional and can be changed later, so we can initialise to empty strings 
        let paynow = Paynow {integration_id, integration_key, returnurl: "", resulturl: "", tokenize: false};
        Ok(paynow)
    }

    /// Create Paynow instance from other Data sources eg dictionary, text file maybe? or JSON i dont know
    /// but it will work simple!!
    /// check the helpers module
    fn from<T>(source: T) -> Result<Paynow, ()>{
        // TODO Fix this nonsense before production use,
        // infact thats why its private for now
        let paynow = Paynow {integration_id: "kung", integration_key: "foo", returnurl: "", resulturl: "", tokenize: false};
        Ok(paynow)
    }

    /// Create Payment 
    pub fn create_payment(&mut self, reference: &'static str, auth_email: &'static str) -> Result<Payment, ()> {
        // initialise payment
        // The unique transaction reference is a mandatory requirement
        // eg  create payment for "invoice 267"
        // all other fields initialized to nada!!
    
        let cart: HashMap<String, isize> = HashMap::new();

        let payment = Payment {
            reference: reference.to_string(),
            items: cart,
            auth_email: auth_email.to_owned(),
            additionalinfo: String::new(),
            payment_method: PaymentMethod::All,
            amount: 0isize,
        };
        Ok(payment)
    }

    // TODO initiate transaction request
    /// Send invokes the Initiate Transaction request to Paynow
    pub fn send(&self, payment: &mut Payment) -> Result<&'static str, &'static str> {
        // URL encoded HTTP request to be returned 
    
        // get Payment amount before posting
        let transaction: HashMap<&'static str, String> = self.build();
        transaction.insert(REFERENCE, payment.reference);
        payment.sum(); 
        transaction.insert(AMOUNT, payment.amount.to_string());
        //optional field
        transaction.insert(ADDITIONAlINFO, payment.additionalinfo);
        // auth email field is optional for non mobile payments
        transaction.insert(AUTHEMAIL, payment.auth_email);
        /* Only when merchant is registered to tokenize payments should field be included
         in transaction, there is an email to find out how, but Im not gonna put it here
         go read the official Paynow docs 
        */
        // TODO iterate over the dictionary, concatenate pairs and serialize to a string
        // 
        transaction.insert(HASH, "RANDOM&FAKEHASH#$%@^$%^9000000909453SD".to_owned());
        // T
        // Ehmm but whats the order of fields??


        Ok("Status=Ok&BrowserUrl=http%3a%2f%2fwwwADEDBDE788862032F1BD82CF3B92DE5D5B40DBB35F1A4FD7D")
    }

    //TODO inititiate express checkout transaction
    /// Send Mobile version transactin , requires email
    pub fn send_mobile(&mut self, payment: &mut Payment, phone: &'static str, method: PaymentMethod) -> Result<HashMap<&'static str, String>, &'static str> {
        // If auth_email does not exist , Throw A tantrum.
        let transaction: HashMap<&'static str, String> = self.build();
        transaction.insert(PHONE, phone.to_owned());
        let method: String = match method {
            PaymentMethod::Ecocash => String::from("ecocash"),
            PaymentMethod::OneMoney => String::from("onemoney"),
            PaymentMethod::Telecash => String::from("telecash"),
            PaymentMethod::Vmc      => String::from("vmc"),
            PaymentMethod::All      => String::from("mobilemoney"),
        };
        transaction.insert(METHOD, method);
        payment.sum();
        transaction.insert(AMOUNT, payment.amount.to_string());
        //optional field
        transaction.insert(ADDITIONAlINFO, payment.additionalinfo);
        // auth email field is optional for non mobile payments
        transaction.insert(AUTHEMAIL, payment.auth_email);
        // etc below
        //check if email exists
        Ok(transaction)
    }

    //TODO initiate passenger ticket transaction
    pub fn init_passenger_ticket_transaction(self, ) -> Result<(),&'static str> {
        //"url endoded http req"
        Ok(())

    } 

}


//

impl Transaction for Paynow {
    /// Makes a minimal transaction
    /// The transaction type i.e Init, Mobile or Passenger Ticket will update and addd required fields
    /// Based on this template, Data Extractors may be used as well soon!!
    fn build(&self) -> HashMap<&'static str, String> {
        let transaction: HashMap<&'static str, String> = HashMap::new();
        transaction.insert(ID, self.integration_id.to_string());
        transaction.insert(REFERENCE, String::new());
        transaction.insert(AMOUNT, String::new());
        transaction.insert(ADDITIONAlINFO, String::new());

        transaction.insert(RETURNURL, self.returnurl.to_owned());
        transaction.insert(RESULTURL, self.resulturl.to_owned());
        // auth email field is optional for non mobile payments
        transaction.insert(AUTHEMAIL, String::new());
        
        /* Only when merchant is registered to tokenize payments should field be included
         in request, there is an email to find out how, but Im not gonna put it here
         go read the official Paynow docs 
        */
        transaction.insert(TOKENIZE, self.tokenize.to_string());
        transaction.insert(STATUS, "Message".to_owned());
        transaction

        // HashMap::new() //dummy
    }
}

/// Helper for composing transactions before posting to Paynow
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    reference: String, // unique identifier for transaction
    items: HashMap<String, isize>,  // Dictionary of items in shopping cart description and amount
    auth_email: String, // Users email address
    additionalinfo: String,
    payment_method: PaymentMethod,
    amount: isize,
}


//Personal notes
// Get data from paynow, analysise and extract required fields for specific transaction

impl Payment {

    pub fn add(&mut self, item: &'static str, amount: isize) {
        //TODO iterate cart and get total amount
        self.items.insert(item.to_owned(), amount);
 
    }

    // remove from cart
    pub fn remove(self, item: &'static str) {
        self.items.remove(item);
    }

    // Payment Total
    fn sum(self) {
        // iterate over cart and update amount field

        for i in self.items.values() {
            self.amount += i;
        }
    } 
    /* Getters & Setters */
    pub fn get_reference(&'static self) -> &'static str {
        &self.reference
    }

    pub fn get_items(&self) -> HashMap<String, isize> {
        self.items
    }

    pub fn get_authemail(&'static self) -> &'static str {
        &self.auth_email
    } 
    pub fn get_additionalinfo(&'static self) -> &'static str {
        &self.additionalinfo
    }
}

//TODO Implement Iterator to later Show Payment info

/// InitResponse Wrapper "Trait" for response from Paynow during transaction initiation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InitResponse {
    success: bool, // Was request Successful?
    instructions: String,
    has_redirect: bool, // Does response have uri to redirect to?
    hash: String, // Hashed transaction from Paynow
    redirect_url: String, // URI where user should be taken to to make payment
    error: String, // message if any
    poll_url: String, // sent from paynow
}

// need to get polls status.....



/*

Initiate a transaction
When the customer is ready to make payment the merchant site must perform an Initiate Transaction request. This request is in the form of an HTTP POST to the URL:

https://www.paynow.co.zw/interface/initiatetransaction

The HTTP POST should include the following fields:

Field	Data Type	Description
id	Integer	Integration ID shown to the merchant in the “3rd Party Site or Link Profile” area of “Receive Payment Links” section of “Sell or Receive” on Paynow.
reference	String	The transaction’s reference on the merchant site, this should be unique to the transaction.
amount	Decimal	Final amount of the transaction to two decimal places (do not include a currency symbol).
additionalinfo	String	(optional) Additional info to be displayed on Paynow to the Customer. This should not include any confidential information.
returnurl	String	The URL on the merchant website the customer will be returned to after the transaction has been processed. It is recommended this URL contains enough information for the merchant site to identify the transaction.
resulturl	String	The URL on the merchant website Paynow will post transaction results to. It is recommended this URL contains enough information for the merchant site to identify the transaction.
authemail	String	(optional) If the field is present and set to an email address Paynow will attempt to auto login the customers email address as an anonymous user. If the email address has a registered account the user will be prompted to login to that account.

N.B. This field is required when initiating Express Checkout Transactions
tokenize	Boolean	(optional) If set to true and the customer uses Visa/Mastercard to complete the transaction, the tokenized payment instrument will be returned in the status update which can be used for recurring payments without requiring further input from the customer.

N.B. A merchant may only use this field if permitted to tokenize payment instruments. Contact [email protected] to apply for this functionality.
status	String	Should be set to “Message” at this stage of the transaction.
hash	String	Details of Hash generation are provided in the Generating Hash section.
*/

//TODO refactor transaction formatting using traits
trait Transaction {
    fn  build(&self) -> HashMap<&'static str, String> ;
}


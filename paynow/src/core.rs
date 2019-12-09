use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    MobileMoney(Mno),
    // Visa / MasterCard
    Vmc(CardIssuer),
    All,

}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Mobile Network Operator enum
pub enum Mno {
    // mapped to 'phone' field,The subscriber numbers of the mobile money wallet to be debited. 
    Ecocash(String),     
    OneMoney(String),
    Telecash(String),
}

/// Card company Visa / Mastercard
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CardIssuer {
    Visa(Card),
    MasterCard(Card),
    // Ko Zimswitch ???
}

// Credit / Debit card information 
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    // Numeric	The Visa/Mastercard PAN
    pub cardnumber: usize,
    // Name printed on front of card
    pub cardname: String,
    // Numeric	3 or 4 digits from rear of card
    pub cardcvv: u16,
	// Numeric	6 digit card expiry date (MMYYYY) e.g. 052018
    pub cardexpiry: u32,
    // String	Customer’s billing address
    pub billingline1: String,
    // String	Not required but will assist with fraud detection
    pub billingline2: String,
    // String	Customer’s billing address city
    pub billingcity: String,
    // String	Not required but will assist with fraud detection
    pub billingprovince: String,
    // String	Customer’s billing address country
    pub billingcountry:  String,
}

/// Paynow type for interacting with paynow api
#[derive( Debug, Serialize, Deserialize, PartialEq)]
pub struct Paynow {
    /* Merchant's endpoints. */
    pub integration_id: &'static str, 
    pub integration_key: &'static str,
    pub return_url: &'static str,
    pub result_url: &'static str, 
    pub tokenize: bool,
}


impl Paynow {
    // Constructor
    pub fn new(integration_id: &'static str, integration_key: &'static str) -> Paynow {
        // return url and result are optional and can be changed later, so we can initialise to empty strings 
        Paynow {integration_id, integration_key, return_url: "", result_url: "", tokenize: false}
    }

    /// Create Payment 
    pub fn create_payment(&mut self, reference: &'static str) -> Payment {
        // initialise payment
        // The unique transaction reference is a mandatory requirement
        // eg  create payment for "invoice 267"
        // all other fields initialized to nothingness!!
    
        let cart: HashMap<String, isize> = HashMap::new();
        let email = String::new();
        Payment {
            reference: reference.to_string(),
            items: cart,
            auth_email: email,
            additionalinfo: String::new(),
            payment_method: PaymentMethod::All,
            total: 0isize,
        } 
    }

    // TODO initiate transaction request
    pub fn init_transaction(self, payment: &mut Payment) -> &'static str {
        // URL encoded HTTP request to be returned 

        // get Payment total before posting
        payment.get_total();
        let transaction_post: HashMap<&'static str, String> = HashMap::with_capacity(8);
        transaction_post.insert("id", self.integration_id.to_string());
        transaction_post.insert("reference", payment.reference);
        //TODO create field in payment type
        // transaction_post.insert("amount", payment.total);
        transaction_post.insert("additionalinfo", payment.additionalinfo);
        transaction_post.insert("returnurl", self.return_url.to_owned());
        transaction_post.insert("resulturl", self.result_url.to_owned());
        transaction_post.insert("authemail", payment.auth_email);
        // Only when merchant permitted 
        if self.tokenize {
            transaction_post.insert("tokenize", "True".to_owned());
        } 
        transaction_post.insert("status", "Message".to_owned());
        //TODO write hash generator helper for this
        transaction_post.insert("hash", "RANDOM&FAKEHASH#$%@^$%^9000000909453SD".to_owned());
        // TODO next is seriaize and url encode this hashmap
        // Ehmm but whats the order of fields??


        "Status=Ok&BrowserUrl=http%3a%2f%2fwwwADEDBDE788862032F1BD82CF3B92DE5D5B40DBB35F1A4FD7D"
    }

    //TODO inititiate express checkout transaction
    pub fn init_express_checkout_transaction(self, ) -> () {
        // write something when your brain is freed.
    }

    //TODO initiate passenger ticket transaction
    pub fn init_passenger_ticket_transaction(self, ) -> () {
        //"url endoded http req"

    } 
}


/// Payment helper "Trait" for composing transaction before posting to Paynow
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub reference: String, // unique identifier for transaction
    pub items: HashMap<String, isize>,  // Dictionary of items in shopping cart description and amountjjj
    pub auth_email: String, // Users email address
    pub additionalinfo: String,
    pub payment_method: PaymentMethod,
    pub total: isize,
}


//Persoanl notes
// Get data from paynow, analysise and extract required fields for specific transaction

impl Payment {

    pub fn add(self, item: &'static str, amount: isize) {
        //TODO iterate cart and get total amount
        self.items.insert(item.to_owned(), amount);
    }

    // remove from cart
    pub fn remove(self, item: &'static str) {
        self.items.remove(item);
    }

    // Payment Total
    fn get_total(self) {
        // iterate over cart and update total field

        for i in self.items.values() {
            self.total += i;
        }
    } 
}

/// InitResponse Wrapper "Trait" for response from Paynow during transaction initiation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InitResponse {
    pub success: bool, // Was request Successful?
    pub instructions: String,
    pub has_redirect: bool, // Does response have uri to redirect to?
    pub hash: String, // Hashed transaction from Paynow
    pub redirect_url: String, // URI where user should be taken to to make payment
    pub error: String, // message if any
    pub poll_url: String, // sent from paynow
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



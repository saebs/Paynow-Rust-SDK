/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 

use std::fmt;

// Paynow Endpoints
pub const URL_INITIATE_TRANSACTION: &'static str = "https://www.paynow.co.zw/interface/initiatetransaction";
pub const URL_INITIATE_EXPRESS_CHECKOUT_TRANSACTION: &'static str = "https://www.paynow.co.zw/interface/remotetransaction";
pub const URL_INITIATE_PASSENGER_TICKET_TRANSACTION: &'static str = "https://www.paynow.co.zw/interface/initiatetickettransaction";

// Paynow fields Directory

pub const RESULTURL: &'static str = "resulturl";
pub const RETURNURL: &'static str = "returnurl";
pub const REFERENCE: &'static str = "reference";        
pub const AMOUNT: &'static str    = "amount";
pub const ID: &'static str        = "id";
pub const ADDITIONAL_INFO: &'static str  = "additionalinfo";
pub const AUTHEMAIL: &'static str       = "authemail";
pub const STATUS: &'static str  = "status";          
pub const ERROR: &'static str  = "error";
pub const TOKENIZE: &'static str = "tokenize";
pub const HASH: &'static str  = "hash";
pub const BROWSERURL: &'static str = "browserurl";
pub const POLLURL: &'static str = "pollurl";
pub const METHOD: &'static str = "method";
pub const PARES: &'static str = "pares";
pub const MD: &'static str = "md";
pub const PAYNOW_REFERENCE: &'static str = "paynowreference";
pub const TOKEN: &'static str = "token";
pub const TOKEN_EXPIRY: &'static str = "tokenexpiry";
pub const PHONE: &'static str = "phone";
// passenger ticket extra fields
pub const PRIMARY_TICKET_NUMBER: &'static str = "primaryticketnumber";
pub const PASSENGER_FIRSTNAME: &'static str = "passengerfirstname";
pub const PASSENGER_LASTNAME: &'static str = "passengerlastname";
pub const PASSENGER_ID: &'static str = "passengerid";
pub const PASSENGER_STATUS: &'static str = "passengerstatus";
pub const PASSENGER_TYPE: &'static str = "passengertype";
pub const FIRST_ARRIVAL_LOCATION_CODE: &'static str = "firstarrivallocationcode";
pub const PNR_NUMBER: &'static str = "pnrnumber";
pub const OFFICE_IATA_NUMBER: &'static str = "officeiatanumber";
pub const ORDER_NUMBER: &'static str = "ordernumber";
pub const PLACE_OF_ISSUE: &'static str = "placeofissue";
pub const DEPARTURE_DATE: &'static str = "departuredate";
pub const DEPARTURE_TIME: &'static str = "departuretime";
pub const ARRIVAL_TIME: &'static str = "arrivaltime";
pub const JOURNEY_TYPE: &'static str = "journeytype";
pub const COMPLETE_ROUTE: &'static str = "completeroute";


//Merchant defaults 
pub const URL_MERCHANT_LOCALHOST: &'static str = "http://localhost";

pub enum Status {
    /// When Initiating Transaction this status is set by Merchant 
    Message,

    /// If the initiate transaction request is successful 
    Ook, // Ok is reserved in rust so Ook will do uyabona. 

    /// If the initiate transaction request failed 
    Error, 

    /// Paid  Transaction paid successfully, the merchant will receive the funds
    /// at next settlement.
    Paid,

    /// Awaiting Delivery Transaction paid successfully, but is sitting in 
    /// suspense waiting on the merchant to confirm delivery of the goods.
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

    /// A card holder is required to complete 3ds Secure payments challenge if using VISA / Mastercard
    Pending3ds,
}

impl fmt::Display for Status {
    fn fmt(&self, field: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Message => write!(field, "Message"),
            Status::Ook => write!(field, "Ok"),
            Status::Error => write!(field, "Error"),
            Status::Paid => write!(field, "Paid"),
            Status::AwaitingDelivery => write!(field, "Awaiting Delivery"),
            Status::Delivered => write!(field, "Delivered"),
            Status::Created => write!(field, "Created"),
            Status::Sent => write!(field, "Sent"),
            Status::Cancelled => write!(field, "Cancelled"),
            Status::Disputed => write!(field, "Disputed"),
            Status::Refunded => write!(field, "Refunded"),
            Status::Pending3ds => write!(field, "Pending 3ds")
        }
    }
}


/// Options for Passenger Ticket Transaction
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Passenger {
    // ADT  Adult	
    Adt,
    // CNN Child
    Cnn,
    // INF Infant
    Inf,
    // YTH youth
    Yth,
    // STU Student
    Stu,
    // SCR Senior Citizen
    Scr,
    // MIL Military
    Mil,
}

///  Mode of payment when using Mobile Money and or Credit/ Debit Cards 
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
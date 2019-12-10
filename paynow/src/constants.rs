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
pub const ADDITIONAlINFO: &'static str  = "additionalinfo";
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
pub const PAYNOWREFERENCE: &'static str = "paynowreference";
pub const TOKEN: &'static str = "token";
pub const TOKENEXPIRY: &'static str = "tokenexpiry";
pub const PHONE: &'static str = "phone";
// passenger ticket extra fields
pub const PRIMARYTICKETNUMBER: &'static str = "primaryticketnumber";
pub const PASSENGERFIRSTNAME: &'static str = "passengerfirstname";
pub const PASSENGERLASTNAME: &'static str = "passengerlastname";
pub const PASSENGERID: &'static str = "passengerid";
pub const PASSENGERSTATUS: &'static str = "passengerstatus";
pub const PASSENGERTYPE: &'static str = "passengertype";
pub const FIRSTARRIVALLOCATIONCODE: &'static str = "firstarrivallocationcode";
pub const PNRNUMBER: &'static str = "pnrnumber";
pub const OFFICEIATANUMBER: &'static str = "officeiatanumber";
pub const ORDERNUMBER: &'static str = "ordernumber";
pub const PLACEOFISSUE: &'static str = "placeofissue";
pub const DEPARTUREDATE: &'static str = "departuredate";
pub const DEPARTURETIME: &'static str = "departuretime";
pub const ARRIVALTIME: &'static str = "arrivaltime";
pub const JOURNEYTYPE: &'static str = "journeytype";
pub const COMPLETEROUTE: &'static str = "completeroute";


//Merchant defaults 
pub const URL_MERCHANT_LOCALHOST: &'static str = "http://localhost";
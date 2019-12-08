
use std::collections::HashMap;

/*Statuses
Status                  Description
“Paid”	                Transaction paid successfully, the merchant will receive the funds at next settlement.
“Awaiting Delivery”	    Transaction paid successfully, but is sitting in suspense waiting on the merchant to confirm delivery of the goods.
“Delivered”	            The user or merchant has acknowledged delivery of the goods but the funds are still sitting in suspense awaiting the 24 hour confirmation window to close.

The following are other possible status settings, 
these will sent to the merchant site if they change in Paynow or if the merchant polls Paynow for the current status:

Status	Description
“Created”	            Transaction has been created in Paynow, but has not yet been paid by the customer.
“Sent”	                Transaction has been created in Paynow and an up stream system, the customer has been referred to that upstream system but has not yet made payment.
“Cancelled”	            The transaction has been cancelled in Paynow and may not be resumed and needs to be recreated.
“Disputed”	            Transaction has been disputed by the Customer and funds are being held in suspense until the dispute has been resolved.
“Refunded”	            Funds were refunded back to the customer.
*/
pub enum Status {
    Paid,
    AwaitingDelivery,
    Delivered,
    Created,
    Sent,
    Cancelled,
    Disputed,
    Refunded,
}

// Passenger Types for Passenger Ticket Transaction

/*
Passenger Types
The following are values that can be used for the passengertype field:

Passenger-Type	Details
ADT	            Adult
CNN	            Child
INF	            Infant
YTH	            Youth
STU	            Student
SCR	            Senior Citizen
MIL	            Military
*/

pub enum Passenger {
    ADT,
    CNN,
    INF,
    YTH,
    STU,
    SCR,
    MIL,
}

pub enum PaymentMethod {
    MobileMoney(Mno),
    Vmc(CardIssuer),
}

//Mobile Network Operator
pub enum Mno {
    Ecocash(String), // mapped to 'phone' field,The subscriber numbers of the mobile money wallet to be debited. 
    Telecash(String),
    OneMoney(String),
}

// Visa / Master
pub enum CardIssuer {
    Visa(Card),
    MasterCard(Card),
}

/*
All	method	String	ecocash = Ecocash mobile money OR onemoney = OneMoney mobile money OR vmc = Visa/Mastercard
Mobile Money	phone	String	The subscriber number of the mobile money wallet to be debited
Visa/Mastercard	cardnumber	Numeric	The Visa/Mastercard PAN
Visa/Mastercard	cardname	String	Name printed on front of card
Visa/Mastercard	cardcvv	Numeric	3 or 4 digits from rear of card
Visa/Mastercard	cardexpiry	Numeric	6 digit card expiry date (MMYYYY) e.g. 052018
Visa/Mastercard	billingline1	String	Customer’s billing address
Visa/Mastercard	billingline2	String	Not required but will assist with fraud detection
Visa/Mastercard	billingcity	String	Customer’s billing address city
Visa/Mastercard	billingprovince	String	Not required but will assist with fraud detection
Visa/Mastercard	billingcountry	String	Customer’s billing address country
*/
pub struct Card {
    pub cardnumber: u32,
    pub cardname: String,
    pub cardcvv: u32,
    pub billingline1: String,
    pub billingline2: String,
    pub billingcity: String,
    pub billingprovince: String,
    pub billingcountry:  String,
}

/// Paynow type for interacting with paynow api
pub struct Paynow {
    /* Merchant's endpoints. */
    integration_id: &'str, 
    pub return_url: &'str,
    pub result_url: &'str, 
}


/// Payment helper "Trait" for composing transaction before posting to Paynow
pub struct Payment {
    pub reference: String, // unique identifier for transaction
    pub items: HashMap,  // Dictionary of items in shopping cart
    pub auth_email: String, // Users email address
}

/// InitResponse
/// Wrapper "Trait" for response from Paynow during transaction initiation
struct InitResponse {
    success: bool, // Was request Successful?
    instructions: String,
    has_redirect: bool, // Does response have uri to redirect to?
    hash: String, // Hashed transaction from Paynow
    redirect_url: String, // URI where user should be taken to to make payment
    error: String, // message if any
    poll_url: String, // sent from paynow
}



/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/
/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/
use serde::{Deserialize, Serialize};
use std::fmt;
// Paynow API Endpoints
pub const _URL_INITIATE_TRANSACTION: &'static str =
    "https://www.paynow.co.zw/interface/initiatetransaction";
pub const _URL_INITIATE_EXPRESS_CHECKOUT_TRANSACTION: &'static str =
    "https://www.paynow.co.zw/interface/remotetransaction";
pub const _URL_INITIATE_PASSENGER_TICKET_TRANSACTION: &'static str =
    "https://www.paynow.co.zw/interface/initiatetickettransaction";
//Merchant defaults
pub const _URL_MERCHANT_LOCALHOST: &'static str = "http://localhost";



#[derive(Serialize, Deserialize,Clone, PartialEq,Debug)]
pub enum Status {
    /// When Initiating Transaction this status is set by Merchant
    Message,

    /// If the initiate transaction request is successful
    // 'Ok' is reserved so we'll use just 'Okay' to reduce ambiguity, go argue with your ancestors.
    Okay,

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

impl Default for Status {

    fn default() -> Self {
        Status::Message
    }
}

impl fmt::Display for Status {
    fn fmt(&self, field: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Message => write!(field, "Message"),
            Status::Okay => write!(field, "Ok"),
            Status::Error => write!(field, "Error"),
            Status::Paid => write!(field, "Paid"),
            Status::AwaitingDelivery => write!(field, "AwaitingDelivery"),
            Status::Delivered => write!(field, "Delivered"),
            Status::Created => write!(field, "Created"),
            Status::Sent => write!(field, "Sent"),
            Status::Cancelled => write!(field, "Cancelled"),
            Status::Disputed => write!(field, "Disputed"),
            Status::Refunded => write!(field, "Refunded"),
            Status::Pending3ds => write!(field, "Pending 3ds"),
        }
    }
}

/// Options for Passenger Ticket Transaction
#[derive(Debug, Serialize, Deserialize)]
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

impl fmt::Display for Passenger {
    fn fmt(&self, field: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Passenger::Adt => write!(field, "Adult"),
            Passenger::Cnn => write!(field, "Child"),
            Passenger::Inf => write!(field, "Infant"),
            Passenger::Mil => write!(field, "Military"),
            Passenger::Scr => write!(field, "Senior Citizen"),
            Passenger::Stu => write!(field, "Student"),
            Passenger::Yth => write!(field, "Youth"),
        }
    }
}

///  Mode of payment when using Mobile Money and or Credit/ Debit Cards
#[derive(Debug, Serialize, Deserialize,Clone , PartialEq )]
pub enum PaymentMethod {
    Ecocash,
    OneMoney,
    Telecash,
    Visa,
    MasterCard,
    Other,
}

impl Default for PaymentMethod {
    fn default() -> Self {
        PaymentMethod::Other
    }
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, method: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Some methods like telecash or onemoney may not be supported yet
            // but have been included , just incase
            PaymentMethod::Ecocash => write!(method, "ecocash"),
            PaymentMethod::OneMoney => write!(method, "onemoney"),
            PaymentMethod::Telecash => write!(method, "telecash"),

            // 'vmc' , Visa/Mastercard
            //Paynow doesnt make the distinction but we do...
            PaymentMethod::Visa => write!(method, "vmc"),
            PaymentMethod::MasterCard => write!(method, "vmc"),
            PaymentMethod::Other => write!(method, "")
        }
    }
}

// Credit / Debit card information
#[derive(Debug, Serialize, Deserialize)]
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
    billingcountry: String,
}



#[cfg(test)]
mod tests {
    #[test]
    fn describe_test() {
    // Prove that 1 ->  ~2 
        assert_eq!(1 , 1);
    }

 /* example successfull Status Response
 Status=Ok&BrowserUrl=http%3a%2f%2fwww.paynow.co.zw%3a7106%2fPayment%2fConfirmPayment%2f1169&PollUrl=http%3a%2f%2fwww.paynow.co.zw%3a7106%2fInterface%2fCheckPayment%2f%3fg uid%3d3cb27f4b-b3ef-4d1f-9178-5e5e62a43995&Hash=8614C21DD93749339906DB35C51B06006B33DC8C192F40DFE2DB6549942C837C4452E1D1333DE9DB7814B278C8B9E3C34D1A76D2F937DEE57502336E0A071412
 */
    // Example of failed iniRequest, Deserialize to InitError
    // Status=Error&Error=Invalid+amount+field
}

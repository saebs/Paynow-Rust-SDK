// Reusables module Generics and Traits
use crate::types::*;
use std::collections::HashMap;
type WasSuccessful = bool;

trait Response {
    ///     Get the original data sent from Paynow
    fn get_data(&self, source: HashMap<&'static str, &'static str>) {}
    ///     Returns the poll URL sent from Paynow
    fn poll_url<T>(&self, uri: T) {}

    fn redirect_link<T>(&self, redirect: T) {}

    fn success(&self) -> WasSuccessful {
        false
    }

    ///     Reads through the response data sent from Paynow
    /// load()
    fn load() {}
}

/*
reference	String	The transaction’s reference on the merchant site.
amount	Decimal	Final amount of the transaction to two decimal places.
paynowreference	String	Reference number for the transaction in Paynow.
pollurl	String	The URL on Paynow the merchant site can poll to confirm the transaction’s current status.
status	String	After payment is complete Paynow will notify the merchant site with one of the Status values.
hash




success: boolean;
  hasRedirect: boolean;
  redirectUrl: String;
  error: String;
  pollUrl: String;
  instructions: String;
  status: String;


*/

pub struct InitResponse<T, V> {
    data: HashMap<T, V>,
    // success: boolean;
    has_redirect: bool,
    redirect_url: String,
    error: String,
    poll_url: String,
    instructions: String,
    status: Status,
}

impl Response for InitResponse<String, String> {
    fn success(&self) -> WasSuccessful {
        match self.status {
            Status::Paid => true,
            Status::AwaitingDelivery => true,
            _ => false,
        }
    }
}

pub struct StatusResponse {
    reference: &'static str,
    amount: &'static str,
    paynow_reference: &'static str,
    pollurl: &'static str,
    status: Status,
    hash: &'static str,
}

impl Response for StatusResponse {
    fn success(&self) -> WasSuccessful {
        match self.status {
            Status::Paid => true,
            Status::AwaitingDelivery => true,
            _ => false,
        }
    }
}

// Reusables module Generics and Traits 
use std::collections::HashMap;
type WasPaid = bool;
type WasSuccessful = bool;

trait Response {
     ///     Get the original data sent from Paynow
    fn get_data(&self, source: HashMap<&'static str, &'static str>) {

    }
   ///     Returns the poll URL sent from Paynow
    fn poll_url<T>(&self, uri: T) {

    }

    fn redirect_link<T>(&self, redirect: T) {

    }

    fn success(&self) -> WasSuccessful {
        false
    }

    ///     Reads through the response data sent from Paynow
    /// load()
    fn load() {

    }
}


pub struct InitResponse<T,V>{
    data: HashMap<T, V>,
    has_redirect: bool,
    was_successful: WasSuccessful ,
}

pub struct IResponse;

impl Response for IResponse {
    fn success(&self) -> WasSuccessful {
        false // for now , infact this has to go
    }
}

pub struct StatusResponse {
    amount: usize,	
    data: HashMap<&'static str, &'static str>,	
    reference: &'static str,	
    was_paid: WasPaid,
    was_successful: WasSuccessful,
}

impl StatusResponse {
    fn paid(&self) -> WasPaid {
        self.was_paid
    }
}

impl Response for StatusResponse {
    fn success(&self) -> WasSuccessful {
        self.was_successful
    }
}
// Reusables module Generics and Traits 
use std::collections::HashMap;

// trait PayNowTransaction {
//     fn send<T>(&self) -> T;

//     fn build<T, K>(&self) -> HashMap<T, K>;

// }


trait Response {
    fn get_data(&self, source: HashMap<&'static str, &'static str>) {

    }

    fn poll_url<T>(&self, uri: T) {

    }
    fn redirect_link<T>(&self, redirect: T) {

    }

    fn success(&self) -> bool {
        false
    }
}


pub struct InitResponse<T,V>{
    data: HashMap<T, V>,
    has_redirect: bool,
    was_successful: bool,
}

pub struct IResponse;

impl Response for IResponse {
    fn success(&self) -> bool {
        false // for now
    }
}

pub struct StatusResponse {
    amount: usize,	
    data: HashMap<&'static str, &'static str>,	
    reference: &'static str,	
    was_paid: bool,
    was_successful: bool,
}
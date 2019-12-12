/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 

// Paynow Rust 

/// Core Paynow functionality 
/// like Creating Payments, Handling, Sending and Responses
pub mod pn;
/// Paynow Transaction & Response fields , API endpoints and common defaults 
pub mod constants;

/// General utilities for e.g. hashing , Data Collection , Sanitising and non core stuff
pub mod helpers;

// client here.....

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

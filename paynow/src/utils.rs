/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 

/* Collection of General helper functions or utilities */

use sha2::{Sha512, Digest};

/// Generate a hash form two UTF8 strings "message" and 'intergration key'
pub fn hash_make<'a>(message: &'a str, intergration_key: &'static str) ->  String {
    // concat to key to end of message
    let mut msg = String::new();
    msg.push_str(message);
    msg.push_str(intergration_key);
    // create sha512 hash of result and convert to uppercase hexadecimal
    let hash: &[u8]= msg.as_ref();
    format!("{:X}", Sha512::digest(hash))
}


// TODO hash_verify

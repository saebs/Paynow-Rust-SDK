/********************************
Copyright (c) Webenchanter
Author: Sabelo Ntabeni 
email: sabelo.n@yandex.com
*******************************/ 

/* Collection of General helper functions or utilities */

use sha2::{Sha512, Digest};
use std::collections::{HashMap, BTreeMap};

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
///Concats some transaction values to one string
// Used a BTree to get some form of ordering guarantees but need to verify if 
// order is important when generating the post
pub fn values_to_string(data: BTreeMap<&str,&str>) -> String {
    // concat values 
    let mut post = String::new();
    for val in data.values() {
        post.push_str(val);
    }
    post
}

// TODO hash_verify

/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/
/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/
/* Collection of General helper functions or utilities */

use sha2::{Digest, Sha512};
use std::collections::{BTreeMap};
use std::num::ParseFloatError;

/// Generate a hash form two UTF8 strings "message" and 'intergration key'
pub fn hash_make<'a>(message: &'a str, intergration_key: &'static str) -> String {
    // concat to key to end of message
    let mut msg = String::new();
    msg.push_str(message);
    msg.push_str(intergration_key);
    // create sha512 hash of result and convert to uppercase hexadecimal
    let hash: &[u8] = msg.as_ref();
    format!("{:X}", Sha512::digest(hash))
}


pub fn to_cents(amt: &str) -> Result<u64, ParseFloatError> {
    // parse each numeric
    let amt = amt.parse::<f64>()?;
    Ok((amt * 100f64) as u64 )
}



///Concats some transaction values to one string
// Used a BTree to get some form of ordering guarantees but need to verify if
// order is important when generating the post

#[allow(dead_code)]
pub fn concat_values_to_str(data: BTreeMap<&str, &str>) -> String {
    // concat values
    let mut post = String::new();
    for val in data.values() {
        post.push_str(val);
    }
    post
}


#[cfg(test)]
mod tests {
    use crate::utils::*;
    #[test]
    fn parse_to_cents() {
    // Prove that 1 ->  ~2 
        assert_eq!(1000u64 , to_cents("10").unwrap());
        assert_eq!(1000u64 , to_cents(&"10".to_string()).unwrap());
    }
}



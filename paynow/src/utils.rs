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
use std::num::ParseFloatError;

/// Generate a hash form two UTF8 strings "message" and 'intergration key'
#[allow(dead_code)]
pub fn hash_gen(message: &str, integration_key: &str) -> Result<String, &'static str> {
    // concat to key to end of message
    let mut msg = String::new();
    msg.push_str(message);
    msg.push_str(integration_key);
    // create sha512 hash of result and convert to uppercase hexadecimal
    let hash: &[u8] = msg.as_ref();
    Ok(format!("{:X}", Sha512::digest(hash)))
}


pub fn to_cents(amt: &str) -> Result<usize, ParseFloatError> {
    // parse each numeric
    let amt = amt.parse::<f64>()?;
    Ok((amt * 100f64) as usize)
}




#[cfg(test)]
mod tests {
    use crate::utils::*;
    #[test]
    fn parse_to_cents() {
    // Prove that 1 ->  ~2 
        assert_eq!(1000usize , to_cents("10").unwrap());
        assert_eq!(1000usize , to_cents(&"10".to_string()).unwrap());
    }

    #[test]
    fn hash_util() {
        let message = "1201TEST REF99.99A test ticket transactionhttp://www.google.com/search?q=returnurlhttp://www.google.com/search?q=resulturlMessage";
        assert_eq!(hash_gen(message, "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977").unwrap(), "2A033FC38798D913D42ECB786B9B19645ADEDBDE788862032F1BD82CF3B92DEF84F316385D5B40DBB35F1A4FD7D5BFE73835174136463CDD48C9366B0749C689");
    }

    #[test]
    #[ignore]
    fn hash_validation() {
        //TODO 27-04-2020
        let integration_key = "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977";
        let eg_response = "status=Ok&browserurl=https%3a%2f%2fstaging.paynow.co.zw%2fPayment%2fConfirmPayment%2f9510&pollurl=https%3a%2f%2fstaging.paynow.co.zw%2fInterface%2fCheckPayment%2f%3fguid%3dc7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let hash_expected = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        assert_eq!(hash_valid(eg_response, integration_key), hash_expected)
    }
}



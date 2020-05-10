//! Helper functions or utilities for library

/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/

use sha2::{Digest, Sha512};
use std::num::ParseFloatError;
use url::form_urlencoded::parse;
use urlencoding::decode;

/// Generate a hash for the initRequest
/// integration key has to be issued by paynow
pub fn hash_gen(values: &str, integration_key: &str) -> Result<String, &'static str> {
    // create sha512 hash of result and convert to uppercase hexadecimal
    let mut hashable = String::from(decode(values).unwrap());
    hashable.push_str(integration_key);

    let mut hash = Sha512::new();
    hash.input(hashable.as_bytes());
    let result = hash.result();
    Ok(format!("{:X}", result))
}

/// helper for parsing initResponse string in key-value format
/// collates all values exept hash value which is seperated
/// output: (non-hash values, hash)
fn parse_response(input: &str) -> (String, String) {
    let input = parse(input.as_bytes());
    let mut values = String::new();
    let mut hash = String::new();
    const HASH_LABEL: &str = "hash";

    for (k, v) in input {
        if k != HASH_LABEL {
            values.push_str(&v);
        } else {
            hash.push_str(&v);
        }
    }
    (values, hash)
}

/// Checks if Response hash value is valid as a security measure
/// Precondition: The message is urlencoded
pub fn is_valid_hash(
    urlencoded_response: &str,
    integration_key: &str,
) -> Result<bool, &'static str> {
    let (message, hash) = parse_response(urlencoded_response);
    let re_hash = hash_gen(&message, integration_key).unwrap();
    Ok(re_hash == hash)
}

/// Parse string input to cents
/// The inner represantion of all money or currency is handled in cents
/// eg an input of 1 ZWL dollar, is represented as 100 cents in memory calculations
// This implementation may change in the Transactions to use other Currency libraries
// its still nice to have this function tho
pub fn to_cents(amt: &str) -> Result<usize, ParseFloatError> {
    // parse each numerick
    let amt = amt.parse::<f64>()?;
    Ok((amt * 100f64) as usize)
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    #[test]
    fn parse_to_cents() {
        // Prove that 1 ->  ~2
        assert_eq!(1000usize, to_cents("10").unwrap());
        assert_eq!(1000usize, to_cents(&"10".to_string()).unwrap());
    }

    #[test]
    fn generates_hash() {
        let integration_key = "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977";
        let hash0 = "2A033FC38798D913D42ECB786B9B19645ADEDBDE788862032F1BD82CF3B92DEF84F316385D5B40DBB35F1A4FD7D5BFE73835174136463CDD48C9366B0749C689";
        let hash1 = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "Okhttps://staging.paynow.co.zw/Payment/ConfirmPayment/9510https://staging.paynow.co.zw/Interface/CheckPayment/?guid=c7ed41da-0159-46da-b428-69549f7704139510";
        let message = "1201TEST REF99.99A test ticket transactionhttp://www.google.com/search?q=returnurlhttp://www.google.com/search?q=resulturlMessage";
        assert_eq!(hash_gen(message, integration_key).unwrap(), hash0);
        assert_eq!(hash_gen(response, integration_key).unwrap(), hash1);
    }

    #[test]
    fn parses_response() {
        // let response = "status=Ok&browserurl=https%3a%2f%2fstaging.paynow.co.zw%2fPayment%2fConfirmPayment%2f9510&pollurl=https%3a%2f%2fstaging.paynow.co.zw%2fInterface%2fCheckPayment%2f%3fguid%3dc7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "status=Ok&browserurl=https://staging.paynow.co.zw/Payment/ConfirmPayment/9510&pollurl=https://staging.paynow.co.zw/Interface/CheckPayment/?guid=c7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let message = "Okhttps://staging.paynow.co.zw/Payment/ConfirmPayment/9510https://staging.paynow.co.zw/Interface/CheckPayment/?guid=c7ed41da-0159-46da-b428-69549f7704139510";
        let hash = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        // assert_eq!((message0.to_string(), hash0.to_string()), parse_response_values(message0));
        assert_eq!(
            (message.to_string(), hash.to_string()),
            parse_response(response)
        );
    }
    #[test]
    fn validates_hash() {
        let integration_key = "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977";
        // hash = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "status=Ok&browserurl=https%3a%2f%2fstaging.paynow.co.zw%2fPayment%2fConfirmPayment%2f9510&pollurl=https%3a%2f%2fstaging.paynow.co.zw%2fInterface%2fCheckPayment%2f%3fguid%3dc7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";

        assert_eq!(is_valid_hash(&response, integration_key).unwrap(), true);
    }
}

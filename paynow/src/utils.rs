//! Helper functions or utilities for library

/********************************
Author: Sabelo Ntabeni
email: sabelo.n@yandex.com
*******************************/

use crypto::sha2::Sha512;
use crypto::digest::Digest;
use std::num::ParseFloatError;

use urlencoding::decode;

/// Generate a hash for the initRequest
/// integration key has to be issued by paynow
pub fn hash_gen(message: &str, integration_key: &str) -> Result<String, &'static str> {
    // create sha512 hash of result and convert to uppercase hexadecimal
    let message = decode(message).unwrap();
    let mut combo = String::from(message);
    combo.push_str(integration_key);
    let mut hash = Sha512::new();
    hash.input_str(combo.as_str());
    let out = hash.result_str();
    Ok(out.to_uppercase())
}

/// helper for parsing initResponse string in key-value format 
/// collates all values exept hash value which is seperated 
/// output: (non-hash values, hash)
fn parse_response_values(input: &str) -> (String, String) {
    let input = decode(input).unwrap();
    let mut values_str = String::new();
    let mut key = String::new();
    let mut hash = String::new();
    const KEY_DELIMITER: char = '&';
    const VALUE_DELIMITER: char = '=';
    const HASH_LABEL: &str = "hash";

    let mut key_mode = true;
    for c in input.chars() {
        // We dont want to collate the '&' or '=' but we need to recognise them
        // to switch state
        // we want to seperate the hash value as well
        match c {
            KEY_DELIMITER => {
                key_mode = true;
                key = String::new();
                continue;
            }

            VALUE_DELIMITER => {
                key_mode = false;
                continue;
            }
            _ => { 
                if !key_mode && key.to_lowercase() != HASH_LABEL {
                    values_str.push(c);
                } else if key_mode 
                {
                    key.push(c);
                } else if key.to_lowercase() == HASH_LABEL {
                    hash.push(c);
                } 
            }
        }

    }

    (values_str, hash)
}

/// Checks if Response hash value is valid as a security measure
/// Assumption: The message has been url decoded
pub fn is_valid_hash(
    response_message: &str,
    integration_key: &str,
) -> Result<String, &'static str> {
    // need to sanitize response string

    let (message, hash0) = parse_response_values(response_message);
    // assert_eq!(one, two);
    let mut message = message;
    message.push_str(integration_key);
    let hash1 = hash_gen(&message, integration_key).unwrap();
    assert_eq!(hash1, hash0);
    Ok(hash1)
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
    fn hash_util() {
        let integration_key = "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977";
        let hash0 = "2A033FC38798D913D42ECB786B9B19645ADEDBDE788862032F1BD82CF3B92DEF84F316385D5B40DBB35F1A4FD7D5BFE73835174136463CDD48C9366B0749C689";
        let hash1 = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "Okhttps://staging.paynow.co.zw/Payment/ConfirmPayment/9510https://staging.paynow.co.zw/Interface/CheckPayment/?guid=c7ed41da-0159-46da-b428-69549f7704139510";
        let message = "1201TEST REF99.99A test ticket transactionhttp://www.google.com/search?q=returnurlhttp://www.google.com/search?q=resulturlMessage";
        assert_eq!(hash_gen(message, integration_key).unwrap(), hash0);
        assert_eq!(hash_gen(response, integration_key).unwrap(), hash1);
    }


    #[test]
    fn utils_parses_response() {
        // let response = "status=Ok&browserurl=https%3a%2f%2fstaging.paynow.co.zw%2fPayment%2fConfirmPayment%2f9510&pollurl=https%3a%2f%2fstaging.paynow.co.zw%2fInterface%2fCheckPayment%2f%3fguid%3dc7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "status=Ok&browserurl=https://staging.paynow.co.zw/Payment/ConfirmPayment/9510&pollurl=https://staging.paynow.co.zw/Interface/CheckPayment/?guid=c7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let message = "Okhttps://staging.paynow.co.zw/Payment/ConfirmPayment/9510https://staging.paynow.co.zw/Interface/CheckPayment/?guidc7ed41da-0159-46da-b428-69549f7704139510";
        let hash = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let hash0 = "2A033FC38798D913D42ECB786B9B19645ADEDBDE788862032F1BD82CF3B92DEF84F316385D5B40DBB35F1A4FD7D5BFE73835174136463CDD48C9366B0749C689";
        let message0 = "1201TEST REF99.99A test ticket transactionhttp://www.google.com/search?q=returnurlhttp://www.google.com/search?q=resulturlMessage";
        assert_eq!((message0.to_string(), hash0.to_string()), parse_response_values(message0));
        assert_eq!((message.to_string(), hash.to_string()), parse_response_values(response));
    }
    #[test]
    fn utils_validates_hash() {
        //TODO 27-04-2020
        let integration_key = "3e9fed89-60e1-4ce5-ab6e-6b1eb2d4f977";
        let hash = "750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        let response = "status=Ok&browserurl=https%3a%2f%2fstaging.paynow.co.zw%2fPayment%2fConfirmPayment%2f9510&pollurl=https%3a%2f%2fstaging.paynow.co.zw%2fInterface%2fCheckPayment%2f%3fguid%3dc7ed41da-0159-46da-b428-69549f770413&paynowreference=9510&hash=750DD0B0DF374678707BB5AF915AF81C228B9058AD57BB7120569EC68BBB9C2EFC1B26C6375D2BC562AC909B3CD6B2AF1D42E1A5E479FFAC8F4FB3FDCE71DF4D";
        assert_eq!(
            is_valid_hash(response, integration_key).unwrap(),
           hash 
        );
    }
}

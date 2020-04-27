/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

use crate::types::*;
use std::collections::HashMap;
type WasSuccessful = bool;

trait initResponse {
    ///     Get the original data sent from Paynow
    fn get_data(&self, source: HashMap<&'static str, &'static str>) {}
    ///     Returns the poll URL sent from Paynow
    fn poll_url(&self, uri: &str) {}

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

*/
#[derive(Serialize, Deserialize, Debug)]
pub struct InitResponse {
    status: Status,
    error: String,
    browseurl: String,
    pollurl: String,
    // if 3ds pending
    field: String,
    acsurl: String,
    pareq: String,
    md: String,
    callbackurl: String,
    hash: String,
}

/*
Whenever the status of a transaction is changed, for example payment made,
the Paynow server will send the following message to the merchant server.
The message will be sent as an HTTP POST to the resulturl specified by the 
merchant when the transaction initiation occurred.



reference	String	The transaction’s reference on the merchant site.
amount	Decimal	Final amount of the transaction to two decimal places.
paynowreference	String	Reference number for the transaction in Paynow.
pollurl	String	The URL on Paynow the merchant site can poll to confirm the transaction’s current status.
status	String	After payment is complete Paynow will notify the merchant site with one of the Status values.
hash
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionStatus {
    reference: String,
    amount: String,
    paynowreference: String,
    pollurl: String,
    status: String,
    hash: String,
}

/*




























Field	Description
AcsUrl	The URL that the challenge request should be sent to when initializing the challenge
PaReq	The payload of the challenge request
MD	The identifier linking the challenge being requested to the card being used for payment
CallbackUrl	The URL on Paynow where the result of the challenge should be sent

*/




#[cfg(test)]
mod tests {
    #[test]
    fn describe_test() {
    // Prove that 1 ->  ~2 
        assert_eq!(1 , 1);
    }
}

// As succesfull 3ds remote response
/*
Status=Ok&PollUrl=http%3a%2f%2fwww.paynow.co.zw%3a7106%2fInterface%2fCheckPayment%2f%3fguid%3d3cb27f4b-b3ef-4d1f-9178-5e5e62a43995& PaynowReference=12345&Hash=8614C21DD93749339906DB35C51B06006B33DC8C192F40DFE2DB6549942C837C4452E1D1333DE9DB7814B278C8B9E3C34D1A76D2F937DEE57502336E0A071412
*/

// A failed request response
/*
Status=Error&Error=ElectronicCommerceIndicator+ThreeDSecure+or+ThreeDSecureAttempted+required
*/
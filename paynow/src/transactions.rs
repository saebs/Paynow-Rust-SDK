use crate::types::*;
use serde::Deserialize;
use serde::Serialize;

/// Initiate Transaction
/// When the customer is ready to make payment the merchant site must
/// perform an Initiate Transaction request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InitTxn {
    pub id: String,
    pub reference: String,
    pub amount: String,
    pub additionalinfo: String,
    pub returnurl: String,
    pub resulturl: String,
    pub authemail: String,
    pub tokenize: String,
    pub status: String,
    pub hash: String,
}

impl InitTxn {
    pub fn new() -> InitTxn {
        InitTxn {
            id: "".to_string(),
            reference: "".to_string(),
            amount: "".to_string(),
            additionalinfo: "".to_string(),
            returnurl: "".to_string(),
            resulturl: "".to_string(),
            authemail: "".to_string(),
            tokenize: "".to_string(),
            status: "".to_string(),
            hash: "".to_string(),
        }
    }
}

impl IntoIterator for InitTxn {
    type Item = String;
    type IntoIter = InitTxnIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        InitTxnIntoIterator {
            inittxn: self,
            index: 0,
        }
    }
}

pub struct InitTxnIntoIterator {
    inittxn: InitTxn,
    index: usize,
}

impl Iterator for InitTxnIntoIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let result = match self.index {
            0 => &self.inittxn.id,
            1 => &self.inittxn.reference,
            2 => &self.inittxn.amount,
            3 => &self.inittxn.additionalinfo,
            4 => &self.inittxn.returnurl,
            5 => &self.inittxn.resulturl,
            6 => &self.inittxn.authemail,
            7 => &self.inittxn.tokenize,
            8 => &self.inittxn.status,
            9 => &self.inittxn.hash,
            _ => return None,
        };
        self.index += 1;
        Some(result.to_string())
    }
}

// NOT CONSUMED ITERATOR soon

/// Express Checkout Transaction
#[derive(Serialize, Deserialize, Debug)]
pub struct InitExpressTxn {
    id: String,
    reference: String,
    amount: String,
    additionalinfo: String,
    returnurl: String,
    resulturl: String,
    tokenize: String,
    status: String,
    method: String,
    phone: String,
    cardnumber: String,
    cardname: String,
    cardcvv: String,
    cardexpiry: String,
    billingline1: String,
    billingline2: String,
    billingcity: String,
    billingprovince: String,
    billingcountry: String,
    hash: String,
}

#[derive(Deserialize, Serialize)]
pub struct Init3ds {
    pareq: String,
    md: String, //As per express checkout response from Paynow
    termurl: String,
}

use serde::Deserialize;
use serde::Serialize;
use crate::properties::*;

/// Initiate Transaction
/// When the customer is ready to make payment the merchant site must
/// perform an Initiate Transaction request.
#[derive(Serialize, Deserialize)]
pub struct InitTxn {
    id: String,
    reference: String,
    amount: String,
    additionalinfo: String,
    returnurl: String,
    resulturl: String,
    tokenize: String,
    status: String,
    hash: String,
}


/// Express Checkout Transaction
#[derive(Serialize, Deserialize)]
pub struct InitExpressTxn {
    id: String,
    reference: String,
    amount: String,
    additionalinfo: String,
    returnurl: String,
    resulturl: String,
    tokenize: String,
    status: String,
    method:	String,
    phone:	String,
	cardnumber: String,
	cardname:	String,	
	cardcvv:     String,	
	cardexpiry:	String, 
	billingline1:	String,
	billingline2:	String,
	billingcity:	String,
	billingprovince:	String,	
	billingcountry:	String, 
    hash: String,
}


#[derive(Deserialize, Serialize)]
pub struct Init3ds {
    pareq: String,
    md: String, //As per express checkout response from Paynow
    termurl: String,
}

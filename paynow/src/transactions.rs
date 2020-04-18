use crate::intf::Transact;
use crate::types::*;
use serde::Deserialize;
//use serde::Serialize;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;

type WebForm = HashMap<&'static str, &'static str>;

#[derive(Debug,PartialEq)]
pub struct Transaction {
    id: i64,
    reference: String,
    amount: i64,
    additionalinfo: String,
    returnurl: String,
    resulturl: String,
    authemail: String,
    tokenize: bool,
    merchanttrace: String,
    status: Status,
    // Extra requrements for Express or Mobile
    method: PaymentMethod,
    phone: String,
    cardnumber: String, //numeric
    cardname: String,
    cardcvv: String,
    cardexpiry: String,
    billingline1: String,
    billingline2: String,
    billingcity: String,
    billingprovince: String,
    billingcountry: String,
    token: String,
    hash: String,
}

impl Transact for Transaction {
    // new
    fn new() -> Self {
        Transaction {
            id: 0,
            reference: String::new(),
            amount: 0,
            additionalinfo: String::new(),
            returnurl: String::new(),
            resulturl: String::new(),
            authemail: String::new(),
            tokenize: false,
            merchanttrace: String::new(),
            status: Status::Message,
            // Extra requrements for Express or Mobile
            method: PaymentMethod::Other,
            phone: String::new(),
            cardnumber: String::new(), //numeric
            cardname: String::new(),
            cardcvv: String::new(),
            cardexpiry: String::new(),
            billingline1: String::new(),
            billingline2: String::new(),
            billingcity: String::new(),
            billingprovince: String::new(),
            billingcountry: String::new(),
            token: String::new(),
            hash: String::new(),
        }
    }

    //init

    /// Formats a transaction to specific urlencoded string for each transaction type 
    /// Init 
    fn init(&self) -> String {

    // Decided to hard encode each to get deterministic behaviour when hashing each transaction type
        let initialisetxn: String = format!("id={}&reference={}&amount={}&additionalinfo={}&returnurl={}&resulturl={}&authemail={}&tokenize={}&merchanttrace={}&status={}&hash={}",
            self.id,
            self.reference ,
            self.amount ,
            self.additionalinfo, 
            self.returnurl ,
            self.resulturl ,
            self.authemail ,
            self.tokenize ,
            self.merchanttrace ,
            self.status ,
            self.hash);

            // could have made a copy and concatenanted but would have been expensive
        let initialisemobile: String = format!("id={}&reference={}&amount={}&additionalinfo={}&returnurl={}&resulturl={}&authemail={}&tokenize={}&merchanttrace={}&status={}&method={}&phone={}&hash={}",
            self.id,
            self.reference ,
            self.amount ,
            self.additionalinfo, 
            self.returnurl ,
            self.resulturl ,
            self.authemail ,
            self.tokenize ,
            self.merchanttrace ,
            self.status ,
            self.method,
            self.phone,
            self.hash);
            
        let initialisevmc: String = format!("id={}&reference={}&amount={}&additionalinfo={}&returnurl={}&resulturl={}&authemail={}&tokenize={}&merchanttrace={}&status={}&method={}&phone={}&cardnumber={}&cardname={}&cardcvv={}&cardexpiry={}&billingline1={}&billingline2={}&billingcity={}&billingprovince={}&billingcountry={}&token={}&hash={}",
            self.id,
            self.reference ,
            self.amount ,
            self.additionalinfo, 
            self.returnurl ,
            self.resulturl ,
            self.authemail ,
            self.tokenize ,
            self.merchanttrace ,
            self.status ,
            self.method,
            self.phone,
            self.cardnumber,
            self.cardname,
            self.cardcvv,
            self.cardexpiry,
            self.billingline1,
            self.billingline2,
            self.billingcity,
            self.billingprovince,
            self.billingcountry,
            self.token,
            self.hash);
        
        
        
        match self.method {
            PaymentMethod::Ecocash | PaymentMethod::OneMoney | PaymentMethod::Telecash => {
                initialisemobile
            },
            PaymentMethod::MasterCard | PaymentMethod::Visa => {
                initialisevmc
            },
            PaymentMethod::Other => initialisetxn,
        }
    }
    
    fn load(data: WebForm) {
        let form_data: WebForm = data;
        for (key, value) in form_data.iter() {
        // sanitize input below, to lower etc
            match key {
            ID => {self.id = value},
            REFERENCE => {self.reference = value},
            AMOUNT => {self.amount = value},
            ADDITIONAL_INFO => {self.additionalinfo = value},
            RETURNURL => {self.returnurl = value},
            RESULTURL => {self.resulturl = value},
            AUTHEMAIL => {self.authemail = value},
            TOKENIZE => {self.tokenize = value},
            MERCHANTTRACE => {self.merchanttrace = value},
            STATUS => {self.status = value},
            METHOD => {self.method = value},
            PHONE => {self.phone = value},
            CARDNUMBER => {self.cardnumber = value},
            CARDNAME => {self.cardname = value},
            CARDCVV => {self.cardcvv = value},
            CARDEXPIRY => {self.cardexpiry = value},
            BILLINGLINE1 => {self.billingline1 = value},
            BILLINGLINE2 => {self.billingline2 = value},
            BILLINGCITY => {self.billingcity = value},
            BILLINGCOUNTRY => {self.billingcountry = value},
            BILLINGPROVINCE => {self.billingprovince = value},
            TOKEN => {self.token = value},
            _ => {},
            
            }
        
        } 
        
    }
}// transaction end



#[derive(Deserialize)]
pub struct Init3ds {
    pareq: String,
    md: String, //As per express checkout response from Paynow
    termurl: String,
}





#[test]
    fn new_transaction1(){
    
           let trans0 = Transaction {
            id: 0,
            reference: String::new(),
            amount: 0,
            additionalinfo: String::new(),
            returnurl: String::new(),
            resulturl: String::new(),
            authemail: String::new(),
            tokenize: false,
            merchanttrace: String::new(),
            status: Status::Message,
            // Extra requrements for Express or Mobile
            method: PaymentMethod::Other,
            phone: String::new(),
            cardnumber: String::new(), //numeric
            cardname: String::new(),
            cardcvv: String::new(),
            cardexpiry: String::new(),
            billingline1: String::new(),
            billingline2: String::new(),
            billingcity: String::new(),
            billingprovince: String::new(),
            billingcountry: String::new(),
            token: String::new(),
            hash: String::new(),
        };
        
        assert_eq!(trans0, Transaction::new());
        

    
    }

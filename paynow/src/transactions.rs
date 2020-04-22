/*
Copyright (C) 2020 by Saziwe PBC sabelo.n@yandex.com

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/
use crate::trxn::Transact;
use crate::types::{PaymentMethod, Status};
use serde::Deserialize;
//use serde::Serialize;
//use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;
use std::fmt::{Debug};

#[derive(Debug, PartialEq)]
pub struct Transaction {
// Paynow Transactions Initialise Transaction, Initialise Express Checkout,
// Initialise Passenger ticket(in near future) are representable by this struct
// 
    id: String,
    reference: String,
    amount: u64,
    additionalinfo: String,
    returnurl: String,
    resulturl: String,
    authemail: String,
    tokenize: bool,
    merchanttrace: String,
    status: Status,
    // Extra requirements for Express Checkout i.e. Mobile or Cards
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

// TODO 
// write helper to pad spaces with '+' character

impl Transact for Transaction {
    // new
    fn new() -> Self {
        Transaction {
            id: String::new(),
            reference: String::new(),
            amount: 0,
            additionalinfo: String::new(),
            returnurl: String::new(),
            resulturl: String::new(),
            authemail: String::new(),
            tokenize: false, // Assumed default: Merchant has to register first and enable manually
            merchanttrace: String::new(),
            status: Status::Message, // This is the default when initiating a request to paynow api
            // Extra requirements for Express or Mobile
            method: PaymentMethod::Other,
            phone: String::new(),
            cardnumber: String::new(), //has to be numeric
            cardname: String::new(),
            cardcvv: String::new(),
            cardexpiry: String::new(),
            billingline1: String::new(),
            billingline2: String::new(),
            billingcity: String::new(),
            billingprovince: String::new(),
            billingcountry: String::new(),
            token: String::new(),
            hash: String::new(), // generated by hash generator
        }
    }


    /// Formats a transaction to specific urlencoded string for each transaction type
    /// This string is the body of the HTTPS request 
    /// Init
    fn init(&self) -> String {
        // Decided to hard encode each transaction type in order 
        // to get deterministic behaviour when hashing each transaction type
        let initialisetxn: String = format!("id={}&reference={}&amount={}&additionalinfo={}&returnurl={}&resulturl={}&authemail={}&tokenize={}&merchanttrace={}&status={}&hash={}",
            self.id,
            self.reference,
            self.amount,
            self.additionalinfo,
            self.returnurl,
            self.resulturl,
            self.authemail,
            self.tokenize,
            self.merchanttrace,
            self.status,
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
            PaymentMethod::Ecocash | PaymentMethod::OneMoney | PaymentMethod::Telecash => 
                initialisemobile.replace(" ", "+") , // urlencoding, padding spaces
            PaymentMethod::MasterCard | PaymentMethod::Visa => initialisevmc.replace(" ", "+") ,
            PaymentMethod::Other => initialisetxn.replace(" ", "+") ,
        }
    }

    fn load(&mut self, data: &HashMap<String, String>) {
    
//     Allocating a new String to create a lookup key!
//     Had to do it "the hard way" - Boondocks 
//         assert_eq!("id", data.get(&format!("_{}", "id")).unwrap());
        self.id = data.get(&format!("_{}", "id")).unwrap().to_string();
        self.reference = data.get(&format!("_{}", "reference")).unwrap().to_string(); 
        self.amount = data.get(&format!("_{}", "amount")).unwrap().parse::<u64>().unwrap();
        self.additionalinfo = data.get(&format!("_{}", "additionalinfo")).unwrap().to_string();
        self.returnurl = data.get(&format!("_{}", "returnurl")).unwrap().to_string();
        self.resulturl = data.get(&format!("_{}", "resulturl")).unwrap().to_string();
        self.authemail = data.get(&format!("_{}", "authemail")).unwrap().to_string();
        self.tokenize = data.get(&format!("_{}", "tokenize")).unwrap().to_string().parse::<bool>().unwrap();
        self.merchanttrace = data.get(&format!("_{}", "merchanttrace")).unwrap().to_string();
        // since this function is for specifically loading merchant transactio data / user input 
        // we will search for the default Message or just set it manually
        let status: String = data.get(&format!("_{}", "Status")).unwrap().to_string();
        let status: Status = match &status[..] {
                        // the input has to be sanitized, tolower case but for now we put two variations
                        "Message" | "message" => Status::Message,
                        "Cancelled" | "cancelled" => Status::Cancelled,// if user cancelled
                        "Pending 3ds" | "pending 3ds" => Status::Pending3ds, // i Think Merchant enables this feature so will be include it
                        _ => Status::Error // anything else at this stage should be an error
                        };
        self.status = status;
        // Payment methods get same treatment as above
        let method: String = data.get(&format!("_{}","method")).unwrap().to_string(); 
        let method: PaymentMethod = match &method[..] {
                            "Ecocash" | "ecocash" => PaymentMethod::Ecocash,
                            "Onemoney" | "onemoney" => PaymentMethod::OneMoney,
                            "Telecash" | "telecash" => PaymentMethod::Telecash,
                            //NB Paynow does not make a distinction between Visa or MasterCard
                            // The API only recognises "vmc" for both types of cards 
                            // This implementation abstract that from the user
                            // The PaymentMethod enum will display or format appropiate API values as required
                            "Visa" | "visa" => PaymentMethod::Visa,
                            "MasterCard" | "mastercard"  => PaymentMethod::MasterCard,
                            //                                                                   _         _
                            // Why Paynow left out Zimswitch to which they do support it beats me \_(- -)_/
                            _  => PaymentMethod::Other,
        };
        self.method = method;
        self.phone = data.get(&format!("_{}", "phone")).unwrap().to_string();
        self.cardnumber = data.get(&format!("_{}","cardnumber")).unwrap().to_string();
        self.cardname = data.get(&format!("_{}","name")).unwrap().to_string();
        self.cardcvv = data.get(&format!("_{}","cardcvv")).unwrap().to_string();
        self.cardexpiry = data.get(&format!("_{}","cardexpiry")).unwrap().to_string();
        self.billingline1 = data.get(&format!("_{}","billingline1")).unwrap().to_string();
        self.billingline2 = data.get(&format!("_{}","billingline2")).unwrap().to_string();
        self.billingcity = data.get(&format!("_{}","billingcity")).unwrap().to_string();
        self.billingprovince = data.get(&format!("_{}","billingprovince")).unwrap().to_string();
        self.billingcountry = data.get(&format!("_{}","billingcountry")).unwrap().to_string();
        self.token = data.get(&format!("_{}","token")).unwrap().to_string();
        // snippet below for later additions e.g. passenger ticket transactions 
        // self. = data.get(&format!("_{}","")).unwrap().to_string();
    } // load end

} // transaction end

#[derive(Deserialize)]
pub struct Init3ds {
    pareq: String,
    md: String, //As per express checkout response from Paynow
    termurl: String,
}


#[cfg(test)]
mod tests {
use crate::types::*;
use crate::transactions::*;

    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
#[test]
fn new_transaction1() {
    let trans0 = Transaction {
        id: String::new(),
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

#[test]
fn init_formats() {
        let mut trxn_basic: Transaction = Transaction {
            id: String::from("7"),
            reference: String::from("001"),
            amount: 100u64,
            additionalinfo: String::from("test transcation"),
            returnurl: String::from("http://www.merchant.com/api"),
            resulturl: String::from("http://www.merchant.com/req"),
            authemail: String::from("anon@buy.com"),
            tokenize: false, // Assumed default: Merchant has to register first and enable manually
            merchanttrace: String::new(),
            status: Status::Message, // This is the default when initiating a request to paynow api
            // Extra requirements for Express or Mobile
            method: PaymentMethod::Other,
            phone: String::from("0777100100"),
            cardnumber: String::from("4000-1000-1000-1000"), //has to be numeric
            cardname: String::from("not ginimbi"),
            cardcvv: String::from("442"),
            cardexpiry: String::from("09/23"),
            billingline1: String::from("ipapo"),
            billingline2: String::new(),
            billingcity: String::from("Harare"),
            billingprovince: String::from("Harare Metropolitant"),
            billingcountry: String::from("ZW"),
            token: String::from("XTK100"),
            hash: String::new(), // generated by hash generator
        };
        
        let mut trxn_express1: Transaction = Transaction {
            id: String::from("7"),
            reference: String::from("001"),
            amount: 100u64,
            additionalinfo: String::from("test transcation"),
            returnurl: String::from("http://www.merchant.com/api"),
            resulturl: String::from("http://www.merchant.com/req"),
            authemail: String::from("anon@buy.com"),
            tokenize: false, // Assumed default: Merchant has to register first and enable manually
            merchanttrace: String::new(),
            status: Status::Message, // This is the default when initiating a request to paynow api
            // Extra requirements for Express or Mobile
            method: PaymentMethod::Ecocash,
            phone: String::from("0777100100"),
            cardnumber: String::from("4000-1000-1000-1000"), //has to be numeric
            cardname: String::from("not ginimbi"),
            cardcvv: String::from("442"),
            cardexpiry: String::from("09/23"),
            billingline1: String::from("ipapo"),
            billingline2: String::new(),
            billingcity: String::from("Harare"),
            billingprovince: String::from("Harare Metropolitant"),
            billingcountry: String::from("ZW"),
            token: String::from("XTK100"),
            hash: String::new(), // generated by hash generator
        };
        
        let trxn_express2: Transaction = Transaction {
            id: String::from("7"),
            reference: String::from("001"),
            amount: 100u64,
            additionalinfo: String::from("test transcation"),
            returnurl: String::from("http://www.merchant.com/api"),
            resulturl: String::from("http://www.merchant.com/req"),
            authemail: String::from("anon@buy.com"),
            tokenize: false, // Assumed default: Merchant has to register first and enable manually
            merchanttrace: String::new(),
            status: Status::Message, // This is the default when initiating a request to paynow api
            // Extra requirements for Express or Mobile
            method: PaymentMethod::MasterCard,
            phone: String::from("0777100100"),
            cardnumber: String::from("4000-1000-1000-1000"), //has to be numeric
            cardname: String::from("not ginimbi"),
            cardcvv: String::from("442"),
            cardexpiry: String::from("09/23"),
            billingline1: String::from("ipapo"),
            billingline2: String::new(),
            billingcity: String::from("Harare"),
            billingprovince: String::from("Harare Metropolitant"),
            billingcountry: String::from("ZW"),
            token: String::from("XTK100"),
            hash: String::new(), // generated by hash generator
        };
            let initialisetxn: String = format!("{}", "id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&hash=");
        
        let initialiseecocash = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=ecocash&phone=0777100100&hash=");
            
        let initialisemcard = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=vmc&phone=0777100100&cardnumber=4000-1000-1000-1000&cardname=not+ginimbi&cardcvv=442&cardexpiry=09/23&billingline1=ipapo&billingline2=&billingcity=Harare&billingprovince=Harare+Metropolitant&billingcountry=ZW&token=XTK100&hash=" ) ; 
            
        let initialiseonemoney = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=onemoney&phone=0777100100&hash=");
        
        let initialisetelecash = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=telecash&phone=0777100100&hash=");

        //         assert_eq!(trxn_basic.init(), String::from("id=7&reference=001&amount=001") )
        assert_eq!(trxn_basic.init(), initialisetxn );
        trxn_basic.method = PaymentMethod::Ecocash;
        assert_eq!(trxn_basic.init(), initialiseecocash );
        trxn_basic.method = PaymentMethod::OneMoney;
        assert_eq!(trxn_basic.init(), initialiseonemoney);
        trxn_basic.method = PaymentMethod::Telecash;
        assert_eq!(trxn_basic.init(), initialisetelecash);
        trxn_basic.method = PaymentMethod::Visa;
        // visa or mastercard should give method value : 'vmc' always
        // which implies visa <=> mastercard
        assert_eq!(trxn_basic.init(), initialisemcard);
        assert_eq!(trxn_express2.init(), initialisemcard);
        
}
    
    
}

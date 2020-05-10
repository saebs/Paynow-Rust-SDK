use crate::trxn::Transact;
use crate::types::{PaymentMethod, Status};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
//use serde::Serialize;
//use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    // Paynow Transactions Initialise Transaction, Initialise Express Checkout,
    // Initialise Passenger ticket(in near future) are representable by this struct
    //
    id: String,
    reference: String,
    // Need to decide on type to handle currency
    // initial thoughts whas to internally hande all money in cents
    // converting all input to cents in the inner representation
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
    index: usize,
}

impl Transaction {
    // creator

    pub fn new() -> Self {
        Transaction {
            ..Default::default()
        }
    }
    // Setters

    pub fn set_reference(&mut self, reference: &str) {
        self.reference = reference.to_string();
    }

    pub fn set_authemail(&mut self, authemail: &str) {
        self.authemail = authemail.to_string();
    }

    pub fn set_phone(&mut self, phone: &str) {
        self.phone = phone.to_string();
    }
    pub fn set_method(&mut self, method: &str) {
        let pmt: PaymentMethod = match &method.to_lowercase()[..] {
            "econet" => PaymentMethod::Ecocash,
            "onemoney" => PaymentMethod::OneMoney,
            "telecash" => PaymentMethod::Telecash,
            "visa" => PaymentMethod::Visa,
            "mastercard" => PaymentMethod::MasterCard,
            _ => PaymentMethod::Other,
        };
        self.method = pmt;
    }

    // Getters

    #[allow(dead_code)]
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    #[allow(dead_code)]
    fn get_reference(&self) -> String {
        format!("{}", self.reference)
    }
    #[allow(dead_code)]
    fn get_amount(&self) -> u64 {
        self.amount
    }

    #[allow(dead_code)]
    fn get_additionalinfo(&self) -> String {
        format!("{}", self.additionalinfo)
    }
}

impl Iterator for Transaction {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.index {
            0 => Some(format!("{}", self.id)),
            1 => Some(format!("{}", self.reference)),
            2 => Some(format!("{}", self.amount)),
            3 => Some(format!("{}", self.additionalinfo)),
            4 => Some(format!("{}", self.returnurl)),
            5 => Some(format!("{}", self.resulturl)),
            6 => Some(format!("{}", self.authemail)),
            7 => Some(format!("{}", self.tokenize)),
            8 => Some(format!("{}", self.merchanttrace)),
            9 => Some(format!("{}", self.status)),
            10 => Some(format!("{}", self.method)),
            11 => Some(format!("{}", self.phone)),
            12 => Some(format!("{}", self.cardnumber)),
            13 => Some(format!("{}", self.cardname)),
            14 => Some(format!("{}", self.cardcvv)),
            15 => Some(format!("{}", self.cardexpiry)),
            16 => Some(format!("{}", self.billingline1)),
            17 => Some(format!("{}", self.billingline2)),
            18 => Some(format!("{}", self.billingcity)),
            19 => Some(format!("{}", self.billingprovince)),
            20 => Some(format!("{}", self.billingcountry)),
            21 => Some(format!("{}", self.token)),
            22 => Some(format!("{}", self.hash)),
            _ => None,
        };
        self.index += 1;
        item
    }
}

impl Transact for Transaction {
    // Consider minimum requirement of invariant Transaction id for each new instance
    fn new() -> Self {
        Transaction {
            ..Default::default()
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
            self.amount,
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
                initialisemobile.replace(" ", "+")
            } // urlencoding spaces
            PaymentMethod::MasterCard | PaymentMethod::Visa => initialisevmc.replace(" ", "+"),
            PaymentMethod::Other => initialisetxn.replace(" ", "+"),
        }
    }

    fn load(&mut self, data: &HashMap<&str, &str>) -> Result<(), String> {
        if data.is_empty() {
            return Err("no data in hashmap".to_string());
        };

        //     Allocating a new String to create a lookup key!
        //     "The easy way or the hard way"? -
        self.id = data.get(&"id").unwrap().to_string();
        self.reference = data.get(&"reference").unwrap().to_string();
        self.amount = data.get(&"amount").unwrap().parse::<u64>().unwrap();
        self.additionalinfo = data.get(&"additionalinfo").unwrap().to_string();
        self.returnurl = data.get(&"returnurl").unwrap().to_string();
        self.resulturl = data.get(&"resulturl").unwrap().to_string();
        self.authemail = data.get(&"authemail").unwrap().to_string();
        // not sure if we should search for token explicity set it from some configuration file
        // ???? Its somewhat of an invariant depending on the merchant if registred or not so
        //...   will ignore tokenize parameter until a reason to include it here comes up
        // self.tokenize = data.get(&"tokenize").unwrap().to_string().parse::<bool>().unwrap();

        self.merchanttrace = data.get(&"merchanttrace").unwrap().to_string();
        /***********************************************************************************/
        // since this function is for specifically loading merchant transaction data / user input
        // we will not search for the status at this stage
        // let status: String = data.get(&"Status").unwrap().to_string();
        // let status: Status = match &status[..] {
        //                 // the input has to be sanitized, tolower case but for now we put two variations
        //                 "Message" | "message" => Status::Message,
        //                 "Cancelled" | "cancelled" => Status::Cancelled,// if user cancelled
        //                 "Pending 3ds" | "pending 3ds" => Status::Pending3ds, // i Think Merchant enables this feature so will be include it
        //                 _ => Status::Error // anything else at this stage should be an error
        //                 };
        // self.status = status;
        /*************************************************************************************/
        let method: String = data.get(&"method").unwrap().to_string();
        let method: PaymentMethod = match &method[..] {
            "Ecocash" | "ecocash" => PaymentMethod::Ecocash,
            "Onemoney" | "onemoney" => PaymentMethod::OneMoney,
            "Telecash" | "telecash" => PaymentMethod::Telecash,
            //NB Paynow does not make a distinction between Visa or MasterCard
            // The API only recognises "vmc" for both types of cards
            // This implementation abstract that from the user
            // The PaymentMethod enum will display or format appropiate API values as required
            "Visa" | "visa" => PaymentMethod::Visa,
            "MasterCard" | "mastercard" => PaymentMethod::MasterCard,
            //                                                                   _         _
            // Why Paynow left out Zimswitch to which they do support it beats me \_(- -)_/
            _ => PaymentMethod::Other,
        };
        self.method = method;
        self.phone = data.get(&"phone").unwrap().to_string();
        self.cardnumber = data.get(&"cardnumber").unwrap().to_string();
        self.cardname = data.get(&"name").unwrap().to_string();
        self.cardcvv = data.get(&"cardcvv").unwrap().to_string();
        self.cardexpiry = data.get(&"cardexpiry").unwrap().to_string();
        self.billingline1 = data.get(&"billingline1").unwrap().to_string();
        self.billingline2 = data.get(&"billingline2").unwrap().to_string();
        self.billingcity = data.get(&"billingcity").unwrap().to_string();
        self.billingprovince = data.get(&"billingprovince").unwrap().to_string();
        self.billingcountry = data.get(&"billingcountry").unwrap().to_string();
        self.token = data.get(&"token").unwrap().to_string();
        // snippet below for later additions e.g. passenger ticket transactions
        // self. = data.get(&format!("_{}","")).unwrap().to_string();
        Ok(())
    } // load end
} // transaction end

/*
Field	Description
PaReq	As per express checkout response from Paynow
MD	As per express checkout response from Paynow
TermUrl	This is the URL on the integrator’s server where the result of the challenge will be posted

*/
/// Pending 3ds post
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Init3ds {
    id: String,
    pareq: String,
    status: Status,
    md: String, //As per express checkout response from Paynow
    termurl: String,
    // hash as well
    hash: String,
}

#[cfg(test)]
mod tests {
    use crate::transactions::*;
    use crate::types::*;
    use std::collections::HashMap;

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
            index: 0usize,
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
            index: 0usize,
        };

        let initialisetxn: String = format!("{}", "id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&hash=");

        let initialiseecocash = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=ecocash&phone=0777100100&hash=");

        let initialisemcard = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=vmc&phone=0777100100&cardnumber=4000-1000-1000-1000&cardname=not+ginimbi&cardcvv=442&cardexpiry=09/23&billingline1=ipapo&billingline2=&billingcity=Harare&billingprovince=Harare+Metropolitant&billingcountry=ZW&token=XTK100&hash=" ) ;

        let initialiseonemoney = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=onemoney&phone=0777100100&hash=");

        let initialisetelecash = String::from("id=7&reference=001&amount=100&additionalinfo=test+transcation&returnurl=http://www.merchant.com/api&resulturl=http://www.merchant.com/req&authemail=anon@buy.com&tokenize=false&merchanttrace=&status=Message&method=telecash&phone=0777100100&hash=");

        //         assert_eq!(trxn_basic.init(), String::from("id=7&reference=001&amount=001") )
        assert_eq!(trxn_basic.init(), initialisetxn);
        trxn_basic.method = PaymentMethod::Ecocash;
        assert_eq!(trxn_basic.init(), initialiseecocash);
        trxn_basic.method = PaymentMethod::OneMoney;
        assert_eq!(trxn_basic.init(), initialiseonemoney);
        trxn_basic.method = PaymentMethod::Telecash;
        assert_eq!(trxn_basic.init(), initialisetelecash);
        trxn_basic.method = PaymentMethod::Visa;
        // visa or mastercard should give method value : 'vmc' always
        // which implies visa <=> mastercard
        assert_eq!(trxn_basic.init(), initialisemcard);
    }

    #[test]
    fn iter_trxn() {
        let mut trxn1: Transaction = Transaction {
            id: String::from("7"),
            reference: String::from("001"),
            amount: 100u64,
            additionalinfo: String::from("lorem test some"),
            returnurl: String::from("http://www.merchant.com/api"),
            resulturl: String::from("http://www.merchant.com/reply"),
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
            billingprovince: String::from("Harare Metropolitan"),
            billingcountry: String::from("ZW"),
            token: String::from("XTK100"),
            hash: String::new(), // generated by hash generator
            index: 0usize,
        };

        // gotta test every field , tjo!!
        assert_eq!(trxn1.next(), Some(String::from("7")));
        assert_eq!(trxn1.next(), Some(String::from("001")));
        assert_eq!(trxn1.next(), Some(String::from("100"))); // needs work in Transaction money
        assert_eq!(trxn1.next(), Some(String::from("lorem test some")));
        assert_eq!(
            trxn1.next(),
            Some(String::from("http://www.merchant.com/api"))
        );
        assert_eq!(
            trxn1.next(),
            Some(String::from("http://www.merchant.com/reply"))
        );
        assert_eq!(trxn1.next(), Some(String::from("anon@buy.com")));
        assert_eq!(trxn1.next(), Some(String::from("false")));
        assert_eq!(trxn1.next(), Some(String::new()));
        assert_eq!(trxn1.next(), Some(String::from("Message")));
        assert_eq!(trxn1.next(), Some(String::new()));
        assert_eq!(trxn1.next(), Some(String::from("0777100100")));
        assert_eq!(trxn1.next(), Some(String::from("4000-1000-1000-1000")));
        assert_eq!(trxn1.next(), Some(String::from("not ginimbi")));
        assert_eq!(trxn1.next(), Some(String::from("442")));
        assert_eq!(trxn1.next(), Some(String::from("09/23")));
        assert_eq!(trxn1.next(), Some(String::from("ipapo")));
        assert_eq!(trxn1.next(), Some(String::new()));
        assert_eq!(trxn1.next(), Some(String::from("Harare")));
        assert_eq!(trxn1.next(), Some(String::from("Harare Metropolitan")));
        assert_eq!(trxn1.next(), Some(String::from("ZW")));
        assert_eq!(trxn1.next(), Some(String::from("XTK100")));
        assert_eq!(trxn1.next(), Some(String::new()));
        assert_eq!(trxn1.next(), None);
    }

    #[test]
    fn getters() {
        let trxn2: Transaction = Transaction {
            id: String::from("7"),
            reference: String::from("001"),
            amount: 100u64,
            additionalinfo: String::from("lorem test some"),
            returnurl: String::from("http://www.merchant.com/api"),
            resulturl: String::from("http://www.merchant.com/reply"),
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
            billingprovince: String::from("Harare Metropolitan"),
            billingcountry: String::from("ZW"),
            token: String::from("XTK100"),
            hash: String::new(), // generated by hash generator
            index: 0usize,
        };

        assert_eq!(trxn2.get_id(), String::from("7"));
        assert_eq!(trxn2.get_reference(), String::from("001"));
        assert_eq!(trxn2.get_amount(), 100u64);
        assert_eq!(trxn2.get_additionalinfo(), String::from("lorem test some"));
    }

    #[test]
    fn loading() {
        let mut tr = Transaction::new();
        let form: HashMap<_, _> = HashMap::new();

        assert_eq!(
            format!("{:?}", tr.load(&form)),
            format!("Err(\"no data in hashmap\")",)
        );
    }
}

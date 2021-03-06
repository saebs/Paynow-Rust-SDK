
#[cfg(test)]
mod tests {
    use crate::core::{Payment, Paynow};
//     use crate::transactions::{Transaction};
//     use crate::types::{PaymentMethod, Paynow,};
    use crate::utils::*;
    use std::collections::{BTreeMap, HashMap};
    
    #[test]
    fn creates_paynow_instance() {
        let paynow: Paynow = Paynow::new();
        let paylater = Paynow {
            integration_id: "",
            integration_key: "",
            returnurl: "",
            resulturl: "",
            tokenize: false,
        };
        assert_eq!(format!("{:?}", paynow), format!("{:?}", paylater));
    }

    #[test]
    fn creates_new_empty_payment() {
        let gimme_my_money: Payment = Payment::new();
        let nah = Payment {
            reference: "",         // unique identifier for transaction
            items: HashMap::new(), // Dictionary of items in shopping cart description and amount
            auth_email: "",        // Users email address
            additionalinfo: "",
            amount: 0,
            requests: String::from("nothing"),
        };

        assert_eq!(format!("{:?}", gimme_my_money), format!("{:?}", nah));
    }

    #[test]
    fn adds_to_basket() {
        let mut tsaona: HashMap<&'static str, usize> = HashMap::new();
        tsaona.insert("chingwa", 1500); // ma Cents usatya!!
        let mut katsapo = Payment::new();
        katsapo.add("chingwa", "15.00");
        assert_eq!(tsaona, katsapo.items);
    }

    #[test]
    fn remove_from_shopping_basket() {
        let mut katsapo = Payment::new();
        katsapo.add("scud", "7.00");
        katsapo.remove("scud");
        assert_eq!(katsapo.items.is_empty(), true);
    }

    #[test]
    fn creates_payement_obj() {
        let mut imbadalo = Paynow::new();
        let mut payment = Payment {
            reference: "000",
            items: HashMap::new(),
            auth_email: "your@email.com",
            additionalinfo: "",
            amount: 0usize,
            requests: String::from("foo"),
        };

        assert_eq!(imbadalo.create_payment("000", "your@email.com"), payment);
        // let just test the sum method here
        assert_eq!(0, payment.sum())
    }
    
    #[test]
    #[ignore]
    fn sends_request() {
        let mut album = Paynow::new();
        let payment = album.create_payment("bulk 001", "buyer@bagi.com");
        album.send(payment);
    }
}

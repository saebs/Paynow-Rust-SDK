// Reusables module Generics and Traits 
use std::collections::HashMap;

struct RegularTransaction ; // TODO

struct MobileMoneyTransaction;

struct PassengerTicketTransaction;

trait PayNowTransaction {
    fn send<T>(&self) -> T;

    fn build<T, K>(&self) -> HashMap<T, K>;

    fn get_data<T>(&self,source: T) ;

}
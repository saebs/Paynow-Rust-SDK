pub trait Transact {
/// creates a new instance of a Transaction
    fn new() -> Self;
    /// format transaction for client
    fn init(&self) -> String;
    fn load<T>(&mut self, data: T) {}
}

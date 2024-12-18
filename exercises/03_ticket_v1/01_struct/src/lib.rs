// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

pub struct Order{
    price: u64,
    quantity: u64,

}

pub trait is_av{
    fn is_available(&self) -> bool;
}

impl is_av for Order {
    fn is_available(&self) -> bool{
        if self.quantity > 0 {
            return true
        }
        else{
            return false
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}

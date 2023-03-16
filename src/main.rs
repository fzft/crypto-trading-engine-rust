#![allow(unused)]

enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = (price % 1.0 * scalar as f64) as u64;

        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

struct Limit {
    price: Price,
    orders: Vec<Order>,
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order {
        Order { size, bid_or_ask }
    }
}

fn main() {
    let price = Price::new(1.23456);
    println!("{:?}", price);
}

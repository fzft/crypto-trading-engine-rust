#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    bids: HashMap<Price, Limit>,
    asks: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => println!("already have limit"),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
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
    let buy_order = Order::new(1.0, BidOrAsk::Bid);
    let buy_order2 = Order::new(2.0, BidOrAsk::Bid);
    let sell_order = Order::new(1.0, BidOrAsk::Ask);
    let mut order_book = OrderBook::new();

    order_book.add_order(4.4, buy_order);
    order_book.add_order(4.4, buy_order2);

    println!("{:?}", order_book)
}

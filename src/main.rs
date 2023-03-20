#![allow(unused)]

use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
use rust_decimal_macros::dec;

mod matching_engine;

fn main() {
    let buy_order2 = Order::new(2.0, BidOrAsk::Bid);
    let sell_order = Order::new(1.0, BidOrAsk::Ask);
    let sell_order2 = Order::new(2.0, BidOrAsk::Ask);
    let mut order_book = OrderBook::new();

    order_book.add_order(dec!(4.5), buy_order2);

    order_book.add_order(dec!(4.5), sell_order);
    order_book.add_order(dec!(4.5), sell_order2);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(1.0, BidOrAsk::Bid);
    let value = engine.place_limit_order(pair, dec!(4.5), buy_order).unwrap();
    println!("value: {:?}", value);
}

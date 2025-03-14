//! Contains a api for interacting with an orderbook
//! Orderbook is persisted on a file
//! And there are the following operations:
//! - GetOrders
//! - Place order
//! - Delete order
//! 
//! It cointains a terminal for entering commands about actions:
//! - List(L)
//! - Buy(B) Quantity Limit -> Returns OrderId
//! - Sell(S) Quantity Limit-> Returns OrderId
//! - Delete Id(D)
//! - Quit(Q)
//! - Help(H)
//! 
//! First argument is the path to open/place the exchange file in
//! exchange .
//! 
//! 
//! 
//! 

use std::path;
use std::error::Error;
use std::str::FromStr;

use order::BuySell;

pub mod order;

/// Contains the parsed command line arguments of the main executable
#[derive(Debug)]
pub struct Config
{
    /// Path of the saved orders file. Should be in csv format
    pub path: path::PathBuf
}

impl Config 
{
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        // v2        
        let path = args.next().ok_or("Failed to get path")?;

        let path = path::PathBuf::from(path).canonicalize().unwrap();
        return Ok(Config{path})
    }
}

/// Lists all orders stored on disk
pub fn list(config: &Config) -> Result<(), Box<dyn Error>>
{
    let orderbook = order::OrderBook::load(&config.path)?;
    println!("Orders:");
    orderbook.orders().iter().for_each(|order| println!("{order:?}") );
    Ok(())
}

fn read_from_input<T>(text_to_print: &str) -> Result<T, Box<dyn Error>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    println!("{text_to_print}:");
    let mut quantity = String::new();
    std::io::stdin().read_line(&mut quantity)?;
    let value: T = quantity.trim().parse::<T>()?;
    Ok(value)
}

/// Creates a new order. Will ask for several inputs, and will then store the 
/// new order to disk
pub fn new_order(config: &Config, buy_sell: BuySell) -> Result<(), Box<dyn Error>>
{
    let mut orderbook = order::OrderBook::load(&config.path)?;
    let quantity: u16 = read_from_input("Quantity(integer)")?;
    let price: f32 = read_from_input("Price(float)")?;
    let order_id = orderbook.create_order(buy_sell, quantity, price);
    println!("Created new order with id {order_id}");
    orderbook.save(&config.path)?;
    
    Ok(())
}

/// Delete order command, which asks for a order_id to delete
pub fn delete(config: &Config) -> Result<(), Box<dyn Error>>
{
    let mut orderbook = order::OrderBook::load(&config.path)?;
    
    let order_id: u32 = read_from_input("OrderId to delete(integer)")?;
    orderbook.delete_order(order_id)?;
    orderbook.save(&config.path)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

}
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


#[derive(Debug)]
pub struct Config
{
    pub path: path::PathBuf
}

impl Config 
{
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        let path = match args.next() {
            Some(arg) => arg,
            None => {
                println!("Input error");
                crate::Action::list_commands();
                return Err("Input error");
            }
        };
        let path = path::PathBuf::from(path).canonicalize().unwrap();
        return Ok(Config{path})
    }
}
pub enum Action
{
    Quit, 
    List,
    Help,
    Buy,
    Sell,
    Delete
}

impl Action
{
    pub fn from_string(string: &str) -> Result<Action, Box<dyn Error>>
    {
        // todo create static map instead
        if string == "Q" || string == "Quit"
        {
            return Ok(Action::Quit);
        }
        else if string == "L" || string == "List"
        {
            return Ok(Action::List);
        }
        else if string == "H" || string == "Help"
        {
            return Ok(Action::Help);
        }
        else if string == "B" || string == "Buy"
        {
            return Ok(Action::Buy);
        }
        else if string == "S" || string == "Sell"
        {
            return Ok(Action::Sell);
        }
        else if string == "D" || string == "Delete"
        {
            return Ok(Action::Delete);
        }
        return Err("Cannot convert {string} to Action")?;
    }

    pub fn list_commands()
    {
        println!();
        println!("Commands(shortcut):");
        println!("Quit(Q) - Quit");
        println!("Help(H) - Lists this help text");
        println!("List(L) - Lists all active orders in the orderbook");
        println!("Buy(B) - Place buy order");
    }
}

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

    #[test]
    fn parse_action() {
        let action: Action = Action::from_string("Q").unwrap();
        assert!(matches!(action, Action::Quit));
    }
}

use core::fmt;
use std::path;
use std::fs;


#[derive(Debug, PartialEq)]
pub enum BuySell
{
    Buy,
    Sell
}

impl fmt::Display for BuySell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

impl BuySell
{
    fn from_str(input: &str) -> Result<BuySell, &'static str>
    {
        if input == "B"{
            return Ok(BuySell::Buy);
        }
        else if input == "S"{
            return Ok(BuySell::Sell);
        }
        Err("Invalid char!")
    }
    fn to_str(buy_sell: &BuySell) -> &'static str
    {
        match buy_sell {
            BuySell::Buy => "B",
            BuySell::Sell => "S"
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Order
{
    pub id: u32,
    pub buy_sell: BuySell,
    pub quantity: u16,
    pub price: f32
}

impl Order
{
    fn serialize(&self) -> String
    {
        // todo, charnge to format string for 
        let mut buf = String::new();
        buf += self.id.to_string().as_str();
        buf += ",";
        buf += BuySell::to_str(&self.buy_sell);
        buf += ",";
        buf += self.quantity.to_string().as_str();
        buf += ",";
        buf += self.price.to_string().as_str();
        buf
    }

    fn deserialize(buf: &str) -> Result<Order, &'static str> {
        let values: Vec<&str> = buf.split(',').collect();
        if values.len() != 4 {
            return Err("Failed to deserialize order from {buf}");
        }
        let id = values.get(0).unwrap().parse::<u32>().expect("Failed to parse orderId from {buf}");
        let buy_sell = BuySell::from_str(values.get(1).unwrap()).expect("Failed to parse buySell from {buf}");
        let quantity = values.get(2).unwrap().parse::<u16>().expect("Failed to parse quantity from {buf}");
        let price = values.get(3).unwrap().parse::<f32>().expect("Failed to parse price from {buf}");
        
        let order = Order{id, buy_sell, quantity, price};
        Ok(order)
    }
}

#[derive(Debug, Default)]
pub struct OrderBook
{
    orders: Vec<Order>
}
use std::error::Error;
impl OrderBook
{
    pub fn new() -> OrderBook {
        return OrderBook{..Default::default()};
    }

    pub fn load(path: &path::PathBuf) -> Result<OrderBook, Box<dyn Error>> {
        if path.is_file() && fs::exists(&path)?{
            let file = fs::read_to_string(&path)?;
            
            return Ok(Self::deserialize(&file)?);
        }

        Ok(Self::new())
    }

    pub fn save(&self, path: &path::PathBuf)-> Result<(), Box<dyn Error>> {
        let buffer: String = self.serialize();
        if path.is_file() {
            if fs::exists(path)? {
                fs::remove_file(path)?;
            }
            fs::write(path, buffer.as_str())?;
        }
        Ok(())
    }

    pub fn orders(&self) -> &Vec<Order> {
        return &self.orders;
    }
    fn serialize(&self) -> String
    {
        let mut buf = String::new();
        for (index, order) in self.orders.iter().enumerate()
        {
            buf += order.serialize().as_str();
            if index != self.orders.len() - 1 {
                buf += "\n";
            }
        }
        buf
    }

    fn deserialize(buf: &str) -> Result<OrderBook, &'static str>
    {
        let mut orderbook = OrderBook{..Default::default()};
        for line in buf.split("\n") {
            if line.len() > 0 { 
                orderbook.orders.push(Order::deserialize(&line)?);
            }
        }
        Ok(orderbook)
    }

    pub fn create_order(&mut self, buy_sell: BuySell, quantity: u16, price: f32) -> u32{
        // just use orderId as max id + 1
        let mut max_order_id: u32 = 0;
        for order in &self.orders {
            if order.id > max_order_id {
                max_order_id = order.id;
            }
        }
        let new_order_id: u32 = max_order_id + 1;
        let new_order = Order{buy_sell, id: new_order_id, quantity, price};
        self.orders.push(new_order);
        new_order_id
    }

    pub fn delete_order(&mut self, order_id: u32) -> Result<(), Box<dyn Error>> {
        let index = self.orders.iter().position(|order: &Order| order.id == order_id).unwrap();
        self.orders.remove(index);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::order;

    use super::*;

    #[test]
    fn serialize_order() {
        let order = Order{id: 1,buy_sell: BuySell::Buy,quantity: 10, price: 3.2};
        let buff: String = order.serialize();
        let order_2 = Order::deserialize(&buff).expect("Failed to deserialize order");
        
        assert_eq!(order, order_2);
    }

    #[test]
    fn serialize_orderbook() {
        let mut orderbook1 = OrderBook::new();
        _ = orderbook1.create_order(BuySell::Sell, 2030, 32.4);
        _ = orderbook1.create_order(BuySell::Buy, 1111, 3040.2);
        let buffer: String = orderbook1.serialize();
        let orderbook2 = OrderBook::deserialize(&buffer).expect("Unable to deserialize orderbook");
        assert_eq!(orderbook1.orders, orderbook2.orders); 
    }
    
    #[test]
    fn deserialize_empty_orderbook() {
        let orderbook1 = OrderBook::new();
        let buffer: String = orderbook1.serialize();
        let orderbook2 = OrderBook::deserialize(&buffer).expect("Unable to deserialize orderbook");
        assert_eq!(orderbook1.orders, orderbook2.orders); 
    }
}

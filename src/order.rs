
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
        format!("{},{},{},{}", self.id, BuySell::to_str(&self.buy_sell), self.quantity, self.price)
    }

    fn deserialize(buf: &str) -> Result<Order, Box<dyn Error>> {
        let mut itr = buf.split(',');
        
        let id = itr.next().unwrap().parse::<u32>().expect("Failed to parse orderId from {buf}");
        let buy_sell = BuySell::from_str(itr.next().unwrap()).expect("Failed to parse buySell from {buf}");
        let quantity = itr.next().unwrap().parse::<u16>().expect("Failed to parse quantity from {buf}");
        let price = itr.next().unwrap().parse::<f32>().expect("Failed to parse price from {buf}");
        
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
        self.orders
            .iter()
            .fold(String::new(), |buffer, order|  buffer + order.serialize().as_str() + "\n")
            .trim_end()
            .to_string()
    }

    fn deserialize(buf: &str) -> Result<OrderBook, &'static str>
    {
        Ok(OrderBook{ orders: 
            buf.split("\n")
                .filter(|line| line.len() > 0)
                .map(|line| Order::deserialize(&line).expect("Failed to deserialize order"))
                .collect()})
    }

    pub fn create_order(&mut self, buy_sell: BuySell, quantity: u16, price: f32) -> u32 {
        
        let new_order_id: u32 = 
            self.orders.iter()
            .max_by_key(|order| order.id)
            .map_or(1, |order| order.id + 1);
        
        let new_order = Order{buy_sell, id: new_order_id, quantity, price};
        self.orders.push(new_order);
        new_order_id
    }

    pub fn delete_order(&mut self, order_id: u32) -> Result<(), Box<dyn Error>> {
        self.orders.retain(|order| order.id != order_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_order() {
        let order = Order{id: 1,buy_sell: BuySell::Buy,quantity: 10, price: 3.2};
        println!("Order: {order:?}");
        let buff: String = order.serialize();
        println!("Serialized order: {buff}");
        let order_2 = Order::deserialize(&buff).expect("Failed to deserialize order");
        println!("Deserialized order {order_2:?}");
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

    #[test]
    fn orderbook_delete() {
        let mut orderbook1 = OrderBook::new();
        
        let order_id1 = orderbook1.create_order(BuySell::Buy, 10, 3.2);
        let order_id2 = orderbook1.create_order(BuySell::Sell, 12, 3.6);
        orderbook1.delete_order(order_id1).expect(format!("Failed to delete newly created order 1 {}", order_id1).as_str());
        assert_eq!(orderbook1.orders.len(), 1); 

        orderbook1.delete_order(order_id2).expect(format!("Failed to delete newly created order 2{}", order_id2).as_str());
        assert!(orderbook1.orders.is_empty(), "Expected empty orderbook after deleting all orders"); 
    }
}

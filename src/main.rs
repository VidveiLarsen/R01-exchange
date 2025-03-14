use std::io::{self, Write};
use std::error::Error;
use std::env;
use exchange::{self, Config, order::BuySell};

/// Represents all the possible commands that are executable
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


fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::build(env::args()).expect("Failed to get config");
    println!("Config: {config:?}");


    // run input loop
    let mut done: bool = false;
    while !done {
        print!(">> ");
        std::io::stdout().flush()?;

        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;
        
        match Action::from_string(input.trim())?
        {
            Action::Quit => {
                done = true;
                println!("Quitting");
            },
            Action::Help => {
                Action::list_commands();
            },
            Action::List => {
                exchange::list(&config)?;
            },
            Action::Buy => {
                exchange::new_order(&config, BuySell::Buy)?;
            } ,
            Action::Sell => {
                exchange::new_order(&config, BuySell::Sell)?;
            } ,
            Action::Delete => {
                exchange::delete(&config)?;
            } 
        }
        println!();
    }
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
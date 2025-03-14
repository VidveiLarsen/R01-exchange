use std::io::{self, Write};
use std::error::Error;
use std::env;
use exchange::{self, Config, Action, order::BuySell};

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
                exchange::Action::list_commands();
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

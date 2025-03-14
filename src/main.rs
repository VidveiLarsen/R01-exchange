use std::io::{self, Write};
use std::error::Error;
use std::env;
use exchange::{self, Config, order::BuySell};
use std::collections::HashMap;

/// Represents all the possible commands that are executable
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Action
{
    Quit, 
    List,
    Help,
    Buy,
    Sell,
    Delete
}

struct Command {
    shortcut: String,
    command: String,
    description: String
}

impl Command {
    fn build(shortcut: &str, command: &str, description: &str) -> Command {
        Command{
            shortcut: shortcut.to_string(), 
            command: command.to_string(), 
            description: description.to_string()
        }
    }
}

fn get_commands() -> HashMap<Action, Command> {
    let mut commands = HashMap::new();
    commands.insert(Action::Quit, Command::build("Q", "Quit", "Exits the application"));
    commands.insert(Action::Help, Command::build("H", "Help", "Prints the list of commands"));
    commands.insert(Action::List, Command::build("L", "List", "Print all orders"));
    commands.insert(Action::Delete, Command::build("D", "Delete", "Delete order, specified by order_id"));
    commands.insert(Action::Buy, Command::build("B", "Buy", "Place buy order"));
    commands.insert(Action::Sell, Command::build("S", "Sell", "Place sale order"));
    commands
}

fn list_commands(commands: &HashMap<Action, Command>) {
    println!("Commands:");
    commands.iter().for_each(|(_, command)| {
        println!("{} - {:6}           {:20}", command.shortcut, command.command, command.description);
    });
}

fn get_action_from_input(input: &str, commands: &HashMap<Action, Command>) -> Action {
    commands.iter()
            .find(|(_, command)|{
                return input.to_lowercase() == command.command.to_lowercase() || 
                input.to_lowercase() == command.shortcut.to_lowercase();
            })
            .map_or_else(|| {
                eprintln!("Invalid command!");
                Action::Help}, 
                |(action, _)| *action)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::build(env::args()).expect("Failed to get config");
    println!("Config: {config:?}");

    let commands = get_commands();

    // run input loop
    let mut done: bool = false;
    while !done {
        print!(">> ");
        std::io::stdout().flush()?;

        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;

        match get_action_from_input(input.trim(), &commands)
        {
            Action::Quit => {
                done = true;
                println!("Quitting");
            },
            Action::Help => {
                list_commands(&commands);
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
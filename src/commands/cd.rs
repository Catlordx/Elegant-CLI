use super::command::Command;
use std::env;

pub struct CdCommand;
impl Command for CdCommand {
    fn execute(&self, args: &[String]) {
        if args.len() != 2 {
            println!("Usage: cd <directory>");
            return;
        }

        if let Err(err) = env::set_current_dir(&args[1]) {
            eprintln!("Error: {}", err);
        }
    }
}
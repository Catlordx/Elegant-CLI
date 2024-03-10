use super::command::Command;
use std::env;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[String]) {
        if let Ok(current_dir) = env::current_dir() {
            if let Some(dir_str) = current_dir.to_str() {
                print!("{}", dir_str);
            }
        }
    }
}

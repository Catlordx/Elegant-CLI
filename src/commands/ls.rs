use super::command::Command;
use std::fs;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, _args: &[String]) {
        match fs::read_dir(".") {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            println!("{}", file_name);
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

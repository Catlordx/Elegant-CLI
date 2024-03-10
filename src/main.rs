// use commands::{cat::CatCommand, cd::CdCommand, command::Command, ls::LsCommand, pwd::PwdCommand};
use commands::command::Command;
use commands::cat::*;
use commands::cd::*;
use commands::ls::*;
use commands::pwd::*;
use std::collections::HashMap;
use std::io::{self, Write};

mod commands;

fn main() {
    let mut commands: HashMap<&str, Box<dyn Command>> = HashMap::new();
    commands.insert("cat", Box::new(CatCommand));
    commands.insert("ls", Box::new(LsCommand));
    commands.insert("pwd", Box::new(PwdCommand));
    commands.insert("cd", Box::new(CdCommand));

    loop {
        // 打印当前工作目录路径
        let pwd_command = commands.get("pwd").unwrap();
        pwd_command.execute(&[]);
        print!(": ");
        // 读取用户输入的命令
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let command_name = &parts[0];

        if let Some(command) = commands.get(command_name.as_str()) {
            command.execute(&parts);
            println!();
        } else {
            println!("Unknown command: {}", command_name);
        }
    }
}

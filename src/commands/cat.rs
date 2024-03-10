use super::command::Command;
use std::{fs, path::{Path, PathBuf}};

pub struct CatCommand;

impl Command for CatCommand {
    fn execute(&self, args: &[String]) {
        if args.len() < 2 {
            println!("Usage: cat <file_path>");
            return;
        }

        let file_path = &args[1];
        let full_path = resolve_path(file_path);

        match fs::read_to_string(&full_path) {
            Ok(contents) => println!("{}", contents),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

fn resolve_path(file_path: &str) -> String {
    let mut full_path = PathBuf::new();

    // 解析文件路径中的相对路径
    let path = Path::new(file_path);

    if path.is_absolute() {
        full_path.push(path);
    } else {
        // 获取当前工作目录
        if let Ok(mut current_dir) = std::env::current_dir() {
            // 处理带有 "../" 的相对路径
            for component in path.components() {
                if let Some(component_str) = component.as_os_str().to_str() {
                    if component_str == ".." {
                        current_dir.pop();
                    } else {
                        current_dir.push(component);
                    }
                } else {
                    eprintln!("Error: Invalid path component.");
                    return String::new();
                }
            }
            full_path.push(current_dir);
        } else {
            eprintln!("Error: Unable to determine current directory.");
            return String::new();
        }
    }

    // 将路径转换为字符串并返回
    if let Some(full_path_str) = full_path.to_str() {
        full_path_str.to_string()
    } else {
        eprintln!("Error: Unable to convert path to string.");
        String::new()
    }
}
use std::io::{self, Write};
use std::env;
use colored::*;
pub mod coreutils;

fn main() {
    loop {
        let mut user_input = String::new();

        let cwd = env::current_dir().unwrap();
        let cwd_str = cwd.display().to_string();

        let home_dir = env::var("HOME").unwrap_or_else(|_| String::new());
        let home_dir_str = if !home_dir.is_empty() {
            home_dir
        } else {
            "/".to_owned()
        };

        let cwd_suffix = if cwd_str.starts_with(&home_dir_str) {
            format!("~{} ", &cwd_str[home_dir_str.len()..])
        } else {
            cwd_str.clone()
        };

        print!("{} {} ", cwd_suffix.bold().cyan(), "\n❯".bold().green());

        io::stdout().flush().expect("Failed to flush line out");

        io::stdin().read_line(&mut user_input).expect("Failed to readline");

        user_input = user_input.trim().to_string(); // Trim any whitespace and convert to string

        match user_input.as_str() {
            "ls" => {
                if let Err(e) = coreutils::ls() {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("echo") => {
                let args: Vec<String> = s.split_whitespace().skip(1).map(|s| s.to_string()).collect();
                coreutils::echo(args);
            },
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
            },
            "pwd" => coreutils::pwd(),
            s if s.starts_with("cat") => {
                let file_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::cat(file_name) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("printf") => {
                let args: Vec<String> = s.split_whitespace().skip(1).map(|s| s.to_string()).collect();
                coreutils::printf(args);
            },
            "date" => {
                coreutils::date();
            },
            s if s.starts_with("mkdir") => {
                let dir_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::mkdir(dir_name) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("rmdir") => {
                let dir_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::rmdir(dir_name) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("mv") => {
                let file_name_one = s.split_whitespace().skip(1).next().unwrap_or("");
                let file_name_two = s.split_whitespace().skip(2).next().unwrap_or("");
                if let Err(e) = coreutils::mv(file_name_one, file_name_two) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("touch") => {
                let file_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::touch(file_name) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("cd") => {
                let dir_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::cd(dir_name) {
                    eprintln!("Error: {}", e);
                }
            },
            s if s.starts_with("rm") => {
                let file_name = s.split_whitespace().skip(1).next().unwrap_or("");
                if let Err(e) = coreutils::rm(file_name) {
                    eprintln!("Error: {}", e);
                }
            },
            "tree" => {
                if let Err(e) = coreutils::tree() {
                    eprintln!("Error: {}", e);
                }
            }
            "quit" => {
                break;
            },
            _ => eprintln!("Invailed command: {} ", user_input),
        }
    }
}


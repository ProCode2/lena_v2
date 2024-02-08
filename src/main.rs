mod lexer;
mod token;
use clap::Parser;
use lexer::Lexer;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use toml;

use token::{Token, TokenType};

#[derive(Debug, Deserialize, Serialize)]
struct ProjectConfig {
    entry: String,
}

/// The new language for web dev gang
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}
fn main() {
    // parse command line arguements
    let _arg = Args::parse();
    // get the current dir this executable was called from
    let current_dir = env::current_dir().expect("curret_dir: can not get current dir");
    let config_path = current_dir.join("lena.toml");
    let config_as_str = fs::read_to_string(config_path)
        .expect("read_to_string: config_path: cant read config file as a string");
    let p_config: ProjectConfig = toml::from_str(&config_as_str).unwrap();

    let entry_file_path = current_dir.join(&p_config.entry);
    let entry_code = fs::read_to_string(entry_file_path)
        .expect("read_to_string: entry_file_path: failed to open file");
    let mut lexer = Lexer::new(entry_code.clone());
    println!("{:?}", p_config);
    println!("{:?}", entry_code);
    println!("{:?}", lexer);
    loop {
        let new_token = lexer.next_token();
        println!("{:?}", new_token);
        if new_token.tokentype == TokenType::EOF || new_token.tokentype == TokenType::ILLEGAL {
            println!("last token: {:?}", new_token);
            break;
        }
    }
}

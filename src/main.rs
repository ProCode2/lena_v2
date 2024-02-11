mod component;
mod lexer;
mod parser;
mod token;
use lexer::Lexer;
use parser::Parser as MyParser;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use toml;

use token::TokenType;

use clap::Parser;

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
    let lx = Lexer::new(entry_code.clone());

    let mut parser = MyParser::new(lx.clone());
    // parser.load_default_components();
    // while parser.current_token.tokentype != TokenType::EOF {
    //     println!("{:#?}", parser.current_token);
    //     parser.next_token();
    // }
    let p = parser.parse_program().unwrap();
    // // println!("{:#?}", p_config);
    // // println!("{:#?}", entry_code);
    println!("{:#?}", p);
    println!("{:#?}", parser.errors);
}

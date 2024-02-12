mod component;
mod lexer;
mod parser;
mod token;
use lexer::Lexer;
use parser::Parser as MyParser;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use toml;

use clap::Parser;

#[derive(Debug, Deserialize, Serialize)]
struct BuildConfig {
    dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectConfig {
    entry: String,
    build: BuildConfig,
}

fn scaffold_build_dir(path: PathBuf, js_obj_str: String) {
    if !path.exists() {
        let _created =
            fs::create_dir(&path).map_err(|err| println!("Can create build dir: reason: {}", err));
    }

    let html_path = path.join("index.html");
    let html_code = r#"<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta http-equiv="X-UA-Compatible" content="ie=edge" />
        <title>Lena - The Great Programming Language</title>
        <link rel="stylesheet" href="./style.css" />
        <link rel="icon" href="./favicon.ico" type="image/x-icon" />
    </head>
    <body>
        <main id="app"></main>
        <script src="./index.js"></script>
    </body>
</html>"#;
    let _created = fs::write(html_path, html_code)
        .map_err(|err| println!("Can not create index.html: reason: {}", err));

    let js_path = path.join("index.js");
    let _created_js = fs::write(
        js_path,
        format!(
            r#"function mount(rep){{console.log("My IR is: ", rep)}};
let ir = {};
mount(ir)"#,
            js_obj_str
        ),
    );
}

/// The new language for web dev gang
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Create modern web applications using cool components and fine grained reactivity powered by wasm"
)]
struct Args {
    /// build the project and store in specified directory
    #[arg(short, long, required(false))]
    build: bool,
}

fn main() {
    // parse command line arguements
    let args = Args::parse();
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
    // while parser.current_token.tokentype != TokenType::EOF {
    //     println!("{:#?}", parser.current_token);
    //     parser.next_token();
    // }
    let p = parser.parse_program().unwrap();
    // // println!("{:#?}", p_config);
    // // println!("{:#?}", entry_code);
    let js_obj = p.to_js_object();
    println!("{:#?}", parser.errors);

    println!("{}", js_obj);
    if args.build {
        scaffold_build_dir(current_dir.join(p_config.build.dir), js_obj);
    }
}

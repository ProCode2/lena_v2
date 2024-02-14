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
        <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
        <title>Lena - The Great Programming Language</title>
        <style>
        /* http://meyerweb.com/eric/tools/css/reset/
           v2.0 | 20110126
           License: none (public domain)
        */

        html, body, div, span, applet, object, iframe,
        h1, h2, h3, h4, h5, h6, p, blockquote, pre,
        a, abbr, acronym, address, big, cite, code,
        del, dfn, em, img, ins, kbd, q, s, samp,
        small, strike, strong, sub, sup, tt, var,
        b, u, i, center,
        dl, dt, dd, ol, ul, li,
        fieldset, form, label, legend,
        table, caption, tbody, tfoot, thead, tr, th, td,
        article, aside, canvas, details, embed,
        figure, figcaption, footer, header, hgroup,
        menu, nav, output, ruby, section, summary,
        time, mark, audio, video {
	margin: 0;
	padding: 0;
	border: 0;
	font-size: 100%;
	font: inherit;
	vertical-align: baseline;
        }
        /* HTML5 display-role reset for older browsers */
        article, aside, details, figcaption, figure,
        footer, header, hgroup, menu, nav, section {
	display: block;
        }
        body {
	line-height: 1;
        }
        ol, ul {
	list-style: none;
        }
        blockquote, q {
	quotes: none;
        }
        blockquote:before, blockquote:after,
        q:before, q:after {
	content: '';
	content: none;
        }
        table {
	border-collapse: collapse;
	border-spacing: 0;
        }
        </style>
        <link rel="icon" href="./favicon.ico" type="image/x-icon" />
    </head>
    <body>
        <main id="app"></main>
        <script type="module" src="./index.js"></script>
    </body>
</html>"#;
    let _created = fs::write(html_path, html_code)
        .map_err(|err| println!("Can not create index.html: reason: {}", err));

    let js_path = path.join("index.js");
    let _created_js = fs::write(
        js_path,
        format!(
            r#"import init, {{ mount }} from './pkg/magic.js';
async function run(ir) {{
    await init();
    mount(ir);
}}
let ir = {};
run(ir);
"#,
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

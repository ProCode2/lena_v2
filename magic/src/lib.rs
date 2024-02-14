use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{self, console, Element, Event};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HtmlTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    #[default]
    DIV,
    NOTAG(String),
    TEXT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    STRING(String),
    NUMBER(i32),
    VECOFNUMBER(Vec<Value>),
    VECOFSTRING(Vec<Value>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Component {
    pub tag: String,
    pub children: Vec<Component>,
    pub value: String,
    // pub info: HashMap<String, Value>, // store css and attributes refactor later
}

#[derive(Debug, Clone)]
pub struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag_name: &str) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let el = document.create_element(tag_name).unwrap();

        Self(el)
    }

    pub fn on(self, event_name: &str, cb: impl FnMut(Event) + 'static) -> Self {
        let cb = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);
        self.0
            .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        self
    }

    pub fn text(self, txt: &str) -> Self {
        self.set_text_content(Some(txt));
        self
    }

    pub fn child(self, e: El) -> Self {
        let _ = self.append_child(&e);
        self
    }
}

fn tagify(tag: &str) -> &str {
    match tag {
        "h1" | "h2" | "h3" | "h4" | "div" | "p" | "text" => tag,
        _ => "div",
    }
}

fn create_dom_from_ir(root: Component) -> El {
    console::log_1(&to_value(&root.tag).unwrap());
    let mut e = El::new(tagify(&root.tag));
    if root.children.len() == 1 && root.children[0].tag == "text" {
        e = e.text(&root.children[0].value);
    } else {
        root.children.into_iter().for_each(|c| {
            e = e.clone().child(create_dom_from_ir(c));
        });
    }
    e
}

#[wasm_bindgen]
pub fn mount(c_tree: JsValue) {
    console::log_1(&c_tree);
    let c_tree: Component = from_value(c_tree)
        .map_err(|err| console::log_1(&to_value("Hello12232").unwrap()))
        .unwrap();
    let node = create_dom_from_ir(c_tree.clone());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document
        .query_selector("#app")
        .expect("document should have body")
        .unwrap();
    let _ = body.append_child(&node).expect("can not add app to page");
}

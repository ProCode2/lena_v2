use std::ops::Deref;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::{self, console, Element, Event};

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
}

#[wasm_bindgen]
pub fn mount() {
    let el = El::new("p").on("click", move |_| {
        console::log_1(&to_value("Hello").unwrap())
    });
    el.set_text_content(Some("Hello click me!"));
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have body");
    let _ = body.append_child(&el).map_err(|err| console::log_1(&err));
}

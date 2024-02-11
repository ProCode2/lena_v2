use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
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
    NO_TAG(String),
    TEXT,
}

const DEFAULT_TAGS: [HtmlTag; 8] = [
    HtmlTag::H1,
    HtmlTag::H2,
    HtmlTag::H3,
    HtmlTag::H4,
    HtmlTag::H5,
    HtmlTag::H6,
    HtmlTag::P,
    HtmlTag::DIV,
];

#[derive(Debug, Default, Clone)]
pub struct Component {
    pub tag: HtmlTag,
    pub children: Vec<Component>,
    pub value: String,
    pub info: HashMap<String, crate::parser::Value>, // store css and attributes refactor later
}

impl Component {
    pub fn new(tag: HtmlTag) -> Self {
        Component {
            tag,
            ..Default::default()
        }
    }

    pub fn new_text(val: String) -> Self {
        Component {
            tag: HtmlTag::TEXT,
            value: val,
            ..Default::default()
        }
    }

    // pub fn get_default_tags() -> Vec<Component> {
    //     let default_map = HashMap::<HtmlTag, Component>::new();
    //     DEFAULT_TAGS.map(|tag| default_map.insert(tag, Component::new(tag)))
    // }

    pub fn tag_from_string(string_tag: &str) -> HtmlTag {
        match string_tag {
            "h1" => HtmlTag::H1,
            "p" => HtmlTag::P,
            "div" => HtmlTag::DIV,
            x => HtmlTag::NO_TAG(x.to_string()),
        }
    }
}

// pub trait Node {
//     fn value(&self) -> String;
// }

// impl Node for Component {
//     fn value(&self) -> String {
//         "".to_string()
//     }
// }

// impl Node for String {
//     fn value(&self) -> String {
//         return self.to_string();
//     }
// }

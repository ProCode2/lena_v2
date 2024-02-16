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
    IMG,
    NOTAG(String),
    TEXT,
}

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

    pub fn tag_from_string(string_tag: &str) -> HtmlTag {
        match string_tag {
            "h1" => HtmlTag::H1,
            "p" => HtmlTag::P,
            "div" => HtmlTag::DIV,
            "img" => HtmlTag::IMG,
            x => HtmlTag::NOTAG(x.to_string()),
        }
    }

    pub fn to_js_object(&self) -> String {
        format!(
            "{{tag: '{}', value: '{}', info: {{ {} }},  children: [{}]}}",
            Component::string_from_tag(self.tag.clone()),
            self.value,
            self.info
                .clone()
                .into_iter()
                .map(|(k, v)| format!("{}: {}", k, Component::string_from_value(v)))
                .collect::<Vec<String>>()
                .join(","),
            self.children
                .iter()
                .map(|c| c.to_js_object())
                .collect::<Vec<String>>()
                .join(",\n")
        )
    }

    fn string_from_tag(tag: HtmlTag) -> String {
        match tag {
            HtmlTag::H1 => "h1".to_string(),
            HtmlTag::H2 => "h2".to_string(),
            HtmlTag::H3 => "h3".to_string(),
            HtmlTag::H4 => "h4".to_string(),
            HtmlTag::H5 => "h5".to_string(),
            HtmlTag::H6 => "h6".to_string(),
            HtmlTag::P => "p".to_string(),
            HtmlTag::DIV => "div".to_string(),
            HtmlTag::NOTAG(x) => x,
            HtmlTag::TEXT => "text".to_string(),
            HtmlTag::IMG => "img".to_string(),
        }
    }

    fn string_from_value(val: crate::parser::Value) -> String {
        match val {
            crate::parser::Value::NUMBER(num) => num.to_string(),
            crate::parser::Value::STRING(s) => format!("\"{}\"", s),
            crate::parser::Value::VECOFNUMBER(vec_num) => {
                format!(
                    "[{}]",
                    vec_num
                        .into_iter()
                        .map(|n| Component::string_from_value(n))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            crate::parser::Value::VECOFSTRING(vec_s) => {
                format!(
                    "[{}]",
                    vec_s
                        .into_iter()
                        .map(|n| Component::string_from_value(n))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
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

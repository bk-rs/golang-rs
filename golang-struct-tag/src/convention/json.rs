use pest::iterators::Pairs;

use crate::{error::ParseError, struct_tag_parser::Rule};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JsonStructTag {
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1259
    Ignored,
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1262
    Normal(JsonStructTagName, Vec<JsonStructTagOption>),
}

pub type JsonStructTagName = Option<String>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JsonStructTagOption {
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1278
    String,
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1300
    Omitempty,
    //
    Unknown(String),
}
impl From<&str> for JsonStructTagOption {
    fn from(s: &str) -> Self {
        match s {
            "string" => Self::String,
            "omitempty" => Self::Omitempty,
            _ => Self::Unknown(s.to_owned()),
        }
    }
}

impl JsonStructTag {
    pub(crate) fn from_json_pairs(mut pairs: Pairs<'_, Rule>) -> Result<Self, ParseError> {
        let name_pair = pairs.next().ok_or(ParseError::Unknown)?;
        let name = name_pair.as_str();
        if name == "-" && pairs.peek().is_none() {
            return Ok(Self::Ignored);
        }
        let name = if name.is_empty() {
            None
        } else {
            Some(name.to_owned())
        };

        let options = pairs
            .filter(|pair| !pair.as_str().is_empty())
            .map(|pair| JsonStructTagOption::from(pair.as_str()))
            .collect();

        Ok(Self::Normal(name, options))
    }
}

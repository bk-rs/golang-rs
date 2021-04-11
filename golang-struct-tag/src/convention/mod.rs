use pest::iterators::Pairs;

use crate::{error::ParseError, struct_tag_parser::Rule};

pub mod json;

use self::json::JsonStructTag;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ConventionStructTag {
    Json(JsonStructTag),
    Unknown(String, String),
}

impl ConventionStructTag {
    pub(crate) fn from_convention_pairs(pairs: Pairs<'_, Rule>) -> Result<Vec<Self>, ParseError> {
        pairs
            .into_iter()
            .map(|pair| {
                let pair = pair.into_inner().next().ok_or(ParseError::Unknown)?;

                match pair.as_rule() {
                    Rule::json => JsonStructTag::from_json_pairs(pair.into_inner()).map(Self::Json),
                    Rule::other => {
                        let mut pairs = pair.into_inner();
                        let key = pairs.next().ok_or(ParseError::Unknown)?;
                        let value = pairs.next().ok_or(ParseError::Unknown)?;
                        Ok(Self::Unknown(
                            key.as_str().to_owned(),
                            value.as_str().to_owned(),
                        ))
                    }
                    _ => Err(ParseError::Unknown),
                }
            })
            .collect()
    }
}

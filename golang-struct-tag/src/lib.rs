use std::{
    cmp::PartialEq,
    collections::HashSet,
    hash::{Hash, Hasher},
    str::{self, FromStr},
};

use golang_parser::{tree_sitter::Node, Parser};
use pest::{iterators::Pairs, Parser as _};

// https://github.com/pest-parser/pest/issues/490#issuecomment-808942497
#[allow(clippy::upper_case_acronyms)]
pub(crate) mod convention_struct_tag_parser;
pub mod json;

use self::convention_struct_tag_parser::{ConventionStructTagParser, Rule};
pub use self::json::{JsonStructTag, JsonStructTagOption};

//
//
//
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StructTag {
    RawStringLiteral(String),
    InterpretedStringLiteral(String),
    Convention(HashSet<ConventionStructTag>),
}
#[derive(Eq, Debug, Clone)]
pub enum ConventionStructTag {
    Json(JsonStructTag),
    Unknown(String, String),
}
impl PartialEq for ConventionStructTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Json(_), Self::Json(_)) => true,
            (Self::Unknown(key, _), Self::Unknown(other_key, _)) => key == other_key,
            _ => false,
        }
    }
}
impl Hash for ConventionStructTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Json(_) => "json".hash(state),
            Self::Unknown(key, _) => key.hash(state),
        }
    }
}
impl StructTag {
    pub fn as_json_struct_tag(&self) -> Option<&JsonStructTag> {
        match self {
            Self::Convention(set) => match set
                .iter()
                .find(|x| x == &&ConventionStructTag::Json(JsonStructTag::Ignored))
            {
                Some(ConventionStructTag::Json(x)) => Some(x),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StructTagParseError {
    #[error("GolangParserError {0:?}")]
    GolangParserError(#[from] golang_parser::Error),
    #[error("NodeMissing {0}")]
    NodeMissing(String),
    #[error("NodeKindUnknown {0}")]
    NodeKindUnknown(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("Unknown")]
    Unknown,
}

impl FromStr for StructTag {
    type Err = StructTagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(format!("type _ struct {{ string {} }};", s))?;
        let source = parser.get_source();
        let root_node = parser.get_root_node();

        let mut cursor = root_node.walk();

        let node_type_declaration = root_node
            .named_children(&mut cursor)
            .find(|node| node.kind() == "type_declaration")
            .ok_or_else(|| StructTagParseError::NodeMissing("type_declaration".to_string()))?;
        let node_type_spec = node_type_declaration
            .named_children(&mut cursor)
            .find(|node| node.kind() == "type_spec")
            .ok_or_else(|| StructTagParseError::NodeMissing("type_spec".to_string()))?;
        let _ = node_type_spec
            .named_child(0)
            .ok_or_else(|| StructTagParseError::NodeMissing("type_spec name".to_string()))?;
        let node_struct_type = node_type_spec
            .named_child(1)
            .ok_or_else(|| StructTagParseError::NodeMissing("type_spec type".to_string()))?;
        let node_field_declaration_list = node_struct_type
            .named_children(&mut cursor)
            .find(|node| node.kind() == "field_declaration_list")
            .ok_or_else(|| {
                StructTagParseError::NodeMissing("field_declaration_list".to_string())
            })?;
        let node_field_declaration = node_field_declaration_list
            .named_children(&mut cursor)
            .find(|node| node.kind() == "field_declaration")
            .ok_or_else(|| StructTagParseError::NodeMissing("field_declaration".to_string()))?;
        let _ = node_field_declaration.named_child(0).ok_or_else(|| {
            StructTagParseError::NodeMissing("field_declaration type".to_string())
        })?;
        let node_field_declaration_tag = node_field_declaration
            .named_child(1)
            .ok_or_else(|| StructTagParseError::NodeMissing("field_declaration tag".to_string()))?;

        match node_field_declaration_tag.kind() {
            "raw_string_literal" => {
                Self::from_raw_string_literal_node(node_field_declaration_tag, source)
            }
            "interpreted_string_literal" => {
                Self::from_interpreted_string_literal_node(node_field_declaration_tag, source)
            }
            _ => Err(StructTagParseError::NodeKindUnknown(
                node_field_declaration_tag.kind().to_owned(),
            )),
        }
    }
}

impl StructTag {
    pub fn from_raw_string_literal_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, StructTagParseError> {
        let s = node
            .utf8_text(source)
            .map_err(StructTagParseError::Utf8Error)?;

        match ConventionStructTagParser::parse(Rule::tag, s) {
            Ok(pairs) => ConventionStructTag::from_pairs(pairs)
                .map(|x| Self::Convention(x.into_iter().collect())),
            Err(_) => Ok(Self::RawStringLiteral(s.to_owned())),
        }
    }

    pub fn from_interpreted_string_literal_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, StructTagParseError> {
        let s = node
            .utf8_text(source)
            .map_err(StructTagParseError::Utf8Error)?;

        Ok(Self::InterpretedStringLiteral(s.to_owned()))
    }
}

impl ConventionStructTag {
    fn from_pairs(mut pairs: Pairs<'_, Rule>) -> Result<Vec<Self>, StructTagParseError> {
        let pair = pairs.next().ok_or(StructTagParseError::Unknown)?;

        match pair.as_rule() {
            Rule::tag => pair
                .into_inner()
                .map(|pair| {
                    let pair = pair
                        .into_inner()
                        .next()
                        .ok_or(StructTagParseError::Unknown)?;

                    match pair.as_rule() {
                        Rule::json => {
                            JsonStructTag::from_json_pairs(pair.into_inner()).map(Self::Json)
                        }
                        Rule::other => {
                            let mut pairs = pair.into_inner();
                            let key = pairs.next().ok_or(StructTagParseError::Unknown)?;
                            let value = pairs.next().ok_or(StructTagParseError::Unknown)?;
                            Ok(Self::Unknown(
                                key.as_str().to_owned(),
                                value.as_str().to_owned(),
                            ))
                        }
                        _ => Err(StructTagParseError::Unknown),
                    }
                })
                .collect(),

            _ => Err(StructTagParseError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            StructTag::RawStringLiteral(r#"`foo"bar`"#.to_owned()),
            r#"`foo"bar`"#.parse()?
        );

        assert_eq!(
            StructTag::InterpretedStringLiteral(r#""foo`bar""#.to_owned()),
            r#""foo`bar""#.parse()?
        );

        assert_eq!(
            StructTag::Convention(
                vec![ConventionStructTag::Unknown(
                    "foo".to_owned(),
                    "bar".to_owned()
                )]
                .into_iter()
                .collect()
            ),
            r#"`foo:"bar"`"#.parse()?
        );

        Ok(())
    }

    #[test]
    fn test_json_struct_tag() {
        assert_eq!(
            StructTag::Convention(
                vec![ConventionStructTag::Json(JsonStructTag::Ignored)]
                    .into_iter()
                    .collect(),
            )
            .as_json_struct_tag(),
            Some(&JsonStructTag::Ignored)
        );

        assert_eq!(
            StructTag::Convention(
                vec![ConventionStructTag::Json(JsonStructTag::Normal(
                    Some("foo".to_owned()),
                    vec![]
                ))]
                .into_iter()
                .collect(),
            )
            .as_json_struct_tag(),
            Some(&JsonStructTag::Normal(Some("foo".to_owned()), vec![]))
        );

        assert_eq!(
            StructTag::Convention(
                vec![ConventionStructTag::Unknown(
                    "foo".to_owned(),
                    "bar".to_owned(),
                )]
                .into_iter()
                .collect(),
            )
            .as_json_struct_tag(),
            None
        );
    }
}

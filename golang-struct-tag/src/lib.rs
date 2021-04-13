use std::str::{self, FromStr};

use pest::{iterators::Pairs, Parser as _};
use tree_sitter::Node;

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
    Convention(Vec<ConventionStructTag>),
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ConventionStructTag {
    Json(JsonStructTag),
    Unknown(String, String),
}

#[derive(thiserror::Error, Debug)]
pub enum StructTagParseError {
    #[error("TreeSitterLanguageError {0}")]
    TreeSitterLanguageError(String),
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("UnsupportedType {0}")]
    UnsupportedType(String),
    #[error("Unknown")]
    Unknown,
}

impl FromStr for StructTag {
    type Err = StructTagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .map_err(|err| StructTagParseError::TreeSitterLanguageError(err.to_string()))?;

        let code = format!("type _ struct {{ string {} }};", s);

        let tree = parser.parse(&code, None).ok_or_else(|| {
            StructTagParseError::TreeSitterParseFailed("Not found tree".to_string())
        })?;
        let mut tree_cursor = tree.walk();
        let source = code.as_bytes();
        let node_source_file = tree.root_node();

        let node_type_declaration = node_source_file
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "type_declaration")
            .ok_or_else(|| {
                StructTagParseError::TreeSitterParseFailed("Not found type_declaration".to_string())
            })?;
        let node_type_spec = node_type_declaration
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "type_spec")
            .ok_or_else(|| {
                StructTagParseError::TreeSitterParseFailed("Not found type_spec".to_string())
            })?;
        let _ = node_type_spec.named_child(0).ok_or_else(|| {
            StructTagParseError::TreeSitterParseFailed("Not found type_spec name".to_string())
        })?;
        let node_struct_type = node_type_spec.named_child(1).ok_or_else(|| {
            StructTagParseError::TreeSitterParseFailed("Not found type_spec type".to_string())
        })?;
        let node_field_declaration_list = node_struct_type
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "field_declaration_list")
            .ok_or_else(|| {
                StructTagParseError::TreeSitterParseFailed(
                    "Not found field_declaration_list".to_string(),
                )
            })?;
        let node_field_declaration = node_field_declaration_list
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "field_declaration")
            .ok_or_else(|| {
                StructTagParseError::TreeSitterParseFailed(
                    "Not found field_declaration".to_string(),
                )
            })?;
        let _ = node_field_declaration.named_child(0).ok_or_else(|| {
            StructTagParseError::TreeSitterParseFailed(
                "Not found field_declaration type".to_string(),
            )
        })?;
        let node_field_declaration_tag =
            node_field_declaration.named_child(1).ok_or_else(|| {
                StructTagParseError::TreeSitterParseFailed(
                    "Not found field_declaration tag".to_string(),
                )
            })?;

        match node_field_declaration_tag.kind() {
            "raw_string_literal" => {
                Self::from_raw_string_literal_node(node_field_declaration_tag, source)
            }
            "interpreted_string_literal" => {
                Self::from_interpreted_string_literal_node(node_field_declaration_tag, source)
            }
            _ => Err(StructTagParseError::UnsupportedType(
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
            Ok(pairs) => ConventionStructTag::from_pairs(pairs).map(Self::Convention),
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
            StructTag::Convention(vec![ConventionStructTag::Unknown(
                "foo".to_owned(),
                "bar".to_owned()
            )]),
            r#"`foo:"bar"`"#.parse()?
        );

        Ok(())
    }
}

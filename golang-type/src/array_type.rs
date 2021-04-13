use std::{num::ParseIntError, str};

use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ArrayType {
    pub length: ArrayLength,
    pub element: Box<Type>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayLength {
    IntLiteral(usize),
    Other(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ArrayTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("IntLiteralValueInvalid {0:?}")]
    IntLiteralValueInvalid(ParseIntError),
}
impl ArrayType {
    pub(crate) fn from_array_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_array_type_length = node.named_child(0).ok_or_else(|| {
            ArrayTypeParseError::TreeSitterParseFailed("Not found array_type length".to_string())
        })?;
        let node_array_type_element = node.named_child(1).ok_or_else(|| {
            ArrayTypeParseError::TreeSitterParseFailed("Not found array_type element".to_string())
        })?;

        let length_str = node_array_type_length
            .utf8_text(source)
            .map_err(ArrayTypeParseError::Utf8Error)?;

        let length = match node_array_type_length.kind() {
            "int_literal" => {
                let length: usize = length_str
                    .parse()
                    .map_err(ArrayTypeParseError::IntLiteralValueInvalid)?;
                ArrayLength::IntLiteral(length)
            }
            _ => ArrayLength::Other(length_str.to_string()),
        };

        let element = Type::from_var_spec_type_node(node_array_type_element, source)?;

        Ok(Self {
            length,
            element: element.into(),
        })
    }
}

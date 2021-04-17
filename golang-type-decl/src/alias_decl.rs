use std::str;

use golang_parser::tree_sitter::Node;
use golang_type_core::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AliasDecl {
    pub name: String,
    pub r#type: Type,
}

#[derive(thiserror::Error, Debug)]
pub enum AliasDeclParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(&'static str),
    #[error("Utf8Error {0:?}")]
    Utf8Error(#[from] str::Utf8Error),
    #[error("TypeParseError {0:?}")]
    TypeParseError(#[from] TypeParseError),
}

impl AliasDecl {
    pub(crate) fn from_type_alias_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, AliasDeclParseError> {
        debug_assert!(node.kind() == "type_alias");

        let node_name = node
            .named_child(0)
            .ok_or(AliasDeclParseError::NodeMissing("name"))?;
        let name = node_name.utf8_text(source)?;

        let node_type = node
            .named_child(1)
            .ok_or(AliasDeclParseError::NodeMissing("type"))?;
        let r#type = Type::from_node(node_type, source)?;

        Ok(Self {
            name: name.to_owned(),
            r#type,
        })
    }
}

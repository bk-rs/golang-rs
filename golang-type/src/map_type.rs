use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MapType {
    pub key: Box<Type>,
    pub value: Box<Type>,
}

#[derive(thiserror::Error, Debug)]
pub enum MapTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl MapType {
    pub(crate) fn from_map_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_map_type_key = node.named_child(0).ok_or_else(|| {
            MapTypeParseError::TreeSitterParseFailed("Not found map_type key".to_string())
        })?;
        let node_map_type_value = node.named_child(1).ok_or_else(|| {
            MapTypeParseError::TreeSitterParseFailed("Not found map_type value".to_string())
        })?;

        let key = Type::from_var_spec_type_node(node_map_type_key, source)?;
        let value = Type::from_var_spec_type_node(node_map_type_value, source)?;

        Ok(Self {
            key: key.into(),
            value: value.into(),
        })
    }
}
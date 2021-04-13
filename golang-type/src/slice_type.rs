use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SliceType {
    pub element: Box<Type>,
}

#[derive(thiserror::Error, Debug)]
pub enum SliceTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl SliceType {
    pub(crate) fn from_slice_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_slice_type_element = node.named_child(0).ok_or_else(|| {
            SliceTypeParseError::TreeSitterParseFailed("Not found slice_type element".to_string())
        })?;

        let element = Type::from_var_spec_type_node(node_slice_type_element, source)?;

        Ok(Self {
            element: element.into(),
        })
    }
}

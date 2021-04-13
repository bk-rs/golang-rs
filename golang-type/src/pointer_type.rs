use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerType(Box<Type>);

#[derive(thiserror::Error, Debug)]
pub enum PointerTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl PointerType {
    pub(crate) fn from_pointer_type_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeParseError> {
        let node_pointer_type_element = node.named_child(1).ok_or_else(|| {
            PointerTypeParseError::TreeSitterParseFailed(
                "Not found pointer_type element".to_string(),
            )
        })?;

        let element = Type::from_var_spec_type_node(node_pointer_type_element, source)?;

        Ok(Self(element.into()))
    }
}

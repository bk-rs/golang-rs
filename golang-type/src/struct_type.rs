use tree_sitter::Node;

use crate::TypeParseError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {}

#[derive(thiserror::Error, Debug)]
pub enum StructTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl StructType {
    pub(crate) fn from_struct_type_node(
        _node: Node,
        _source: &[u8],
    ) -> Result<Self, TypeParseError> {
        unimplemented!()
    }
}

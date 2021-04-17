use tree_sitter::Node;

use crate::TypeParseError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FunctionType {}

#[derive(thiserror::Error, Debug)]
pub enum FunctionTypeParseError {}
impl FunctionType {
    pub(crate) fn from_function_type_node(
        _node: Node,
        _source: &[u8],
    ) -> Result<Self, TypeParseError> {
        // TODO
        Ok(Self {})
    }
}

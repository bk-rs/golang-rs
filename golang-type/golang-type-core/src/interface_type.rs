use golang_parser::tree_sitter::Node;

use crate::TypeParseError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InterfaceType {}

#[derive(thiserror::Error, Debug)]
pub enum InterfaceTypeParseError {}
impl InterfaceType {
    pub(crate) fn from_interface_type_node(
        _node: Node,
        _source: &[u8],
    ) -> Result<Self, TypeParseError> {
        // TODO
        Ok(Self {})
    }
}

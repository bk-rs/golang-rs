use golang_parser::tree_sitter::Node;

use crate::TypeParseError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChannelType {}

#[derive(thiserror::Error, Debug)]
pub enum ChannelTypeParseError {}
impl ChannelType {
    pub(crate) fn from_channel_type_node(
        _node: Node,
        _source: &[u8],
    ) -> Result<Self, TypeParseError> {
        // TODO
        Ok(Self {})
    }
}

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt as _};
use tree_sitter::Node;

use crate::TypeParseError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChannelType {}

#[derive(thiserror::Error, Debug)]
pub enum ChannelTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl ChannelType {
    pub(crate) fn from_channel_type_node(
        _node: Node,
        _source: &[u8],
    ) -> Result<Self, TypeParseError> {
        unimplemented!()
    }
}

impl ToTokens for ChannelType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let err = "impl ToTokens for ChannelType is unsupported";
        tokens.append_all(quote!(compile_error!(#err)))
    }
}

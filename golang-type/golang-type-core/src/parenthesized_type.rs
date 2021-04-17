use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt as _};
use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParenthesizedType(pub Box<Type>);

#[derive(thiserror::Error, Debug)]
pub enum ParenthesizedTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl ParenthesizedType {
    pub(crate) fn from_parenthesized_type_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeParseError> {
        let node_parenthesized_type_element = node.named_child(0).ok_or_else(|| {
            ParenthesizedTypeParseError::TreeSitterParseFailed(
                "Not found parenthesized_type element".to_string(),
            )
        })?;

        let element = Type::from_node(node_parenthesized_type_element, source)?;

        Ok(Self(element.into()))
    }
}

impl ToTokens for ParenthesizedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let element = &self.0;
        tokens.append_all(quote!(#element));
    }
}

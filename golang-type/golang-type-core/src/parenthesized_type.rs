use golang_parser::tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParenthesizedType(pub Box<Type>);

#[derive(thiserror::Error, Debug)]
pub enum ParenthesizedTypeParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(String),
}
impl ParenthesizedType {
    pub(crate) fn from_parenthesized_type_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeParseError> {
        let node_parenthesized_type_element = node.named_child(0).ok_or_else(|| {
            ParenthesizedTypeParseError::NodeMissing("parenthesized_type element".to_string())
        })?;

        let element = Type::from_node(node_parenthesized_type_element, source)?;

        Ok(Self(element.into()))
    }
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::ParenthesizedType;

    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for ParenthesizedType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let element = &self.0;
            tokens.append_all(quote!(#element));
        }
    }
}

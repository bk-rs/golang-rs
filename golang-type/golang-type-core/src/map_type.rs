use golang_parser::tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MapType {
    pub key: Box<Type>,
    pub value: Box<Type>,
}

#[derive(thiserror::Error, Debug)]
pub enum MapTypeParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(String),
}
impl MapType {
    pub(crate) fn from_map_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_map_type_key = node
            .named_child(0)
            .ok_or_else(|| MapTypeParseError::NodeMissing("map_type key".to_string()))?;
        let node_map_type_value = node
            .named_child(1)
            .ok_or_else(|| MapTypeParseError::NodeMissing("map_type value".to_string()))?;

        let key = Type::from_node(node_map_type_key, source)?;
        let value = Type::from_node(node_map_type_value, source)?;

        Ok(Self {
            key: key.into(),
            value: value.into(),
        })
    }
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::MapType;

    use proc_macro2::{Punct, Spacing, TokenStream};
    use quote::{quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for MapType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(quote!(::std::collections::HashMap));
            tokens.append(Punct::new('<', Spacing::Alone));
            let key = &self.key;
            tokens.append_all(quote!(#key));
            tokens.append(Punct::new(',', Spacing::Alone));
            let value = &self.value;
            tokens.append_all(quote!(#value));
            tokens.append(Punct::new('>', Spacing::Alone));
        }
    }
}

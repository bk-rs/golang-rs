use std::{num::ParseIntError, str};

use golang_parser::tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ArrayType {
    pub length: ArrayLength,
    pub element: Box<Type>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayLength {
    IntLiteral(usize),
    Other(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ArrayTypeParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("IntLiteralValueInvalid {0:?}")]
    IntLiteralValueInvalid(ParseIntError),
}
impl ArrayType {
    pub(crate) fn from_array_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_array_type_length = node
            .named_child(0)
            .ok_or_else(|| ArrayTypeParseError::NodeMissing("array_type length".to_string()))?;
        let node_array_type_element = node
            .named_child(1)
            .ok_or_else(|| ArrayTypeParseError::NodeMissing("array_type element".to_string()))?;

        let length_str = node_array_type_length
            .utf8_text(source)
            .map_err(ArrayTypeParseError::Utf8Error)?;

        let length = match node_array_type_length.kind() {
            "int_literal" => {
                let length: usize = length_str
                    .parse()
                    .map_err(ArrayTypeParseError::IntLiteralValueInvalid)?;
                ArrayLength::IntLiteral(length)
            }
            _ => ArrayLength::Other(length_str.to_string()),
        };

        let element = Type::from_node(node_array_type_element, source)?;

        Ok(Self {
            length,
            element: element.into(),
        })
    }
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::ArrayType;

    use proc_macro2::{Punct, Spacing, TokenStream};
    use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for ArrayType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let vec_ident = format_ident!("{}", "Vec");
            tokens.append_all(quote!(#vec_ident));
            tokens.append(Punct::new('<', Spacing::Alone));
            let element = &self.element;
            tokens.append_all(quote!(#element));
            tokens.append(Punct::new('>', Spacing::Alone));
        }
    }
}

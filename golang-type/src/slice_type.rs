use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use tree_sitter::Node;

use crate::{Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SliceType {
    pub element: Box<Type>,
}

#[derive(thiserror::Error, Debug)]
pub enum SliceTypeParseError {
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
}
impl SliceType {
    pub(crate) fn from_slice_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_slice_type_element = node.named_child(0).ok_or_else(|| {
            SliceTypeParseError::TreeSitterParseFailed("Not found slice_type element".to_string())
        })?;

        let element = Type::from_node(node_slice_type_element, source)?;

        Ok(Self {
            element: element.into(),
        })
    }
}

impl ToTokens for SliceType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vec_ident = format_ident!("{}", "Vec");
        tokens.append_all(quote!(#vec_ident));
        tokens.append(Punct::new('<', Spacing::Alone));
        let element = &self.element;
        tokens.append_all(quote!(#element));
        tokens.append(Punct::new('>', Spacing::Alone));
    }
}

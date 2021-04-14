pub use golang_type_name;

use std::str::{self, FromStr};

use golang_type_name::{TypeName, TypeNameParseError};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt as _};
use tree_sitter::Node;

pub mod array_type;
pub mod map_type;
pub mod pointer_type;
pub mod slice_type;

pub use self::array_type::{ArrayLength, ArrayType, ArrayTypeParseError};
pub use self::map_type::{MapType, MapTypeParseError};
pub use self::pointer_type::{PointerType, PointerTypeParseError};
pub use self::slice_type::{SliceType, SliceTypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    TypeName(TypeName),
    ArrayType(ArrayType),
    PointerType(PointerType),
    SliceType(SliceType),
    MapType(MapType),
}

#[derive(thiserror::Error, Debug)]
pub enum TypeParseError {
    #[error("TreeSitterLanguageError {0}")]
    TreeSitterLanguageError(String),
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
    #[error("UnsupportedType {0}")]
    UnsupportedType(String),
    //
    #[error("TypeNameParseError {0:?}")]
    TypeNameParseError(#[from] TypeNameParseError),
    #[error("ArrayTypeParseError {0:?}")]
    ArrayTypeParseError(#[from] ArrayTypeParseError),
    #[error("MapTypeParseError {0:?}")]
    MapTypeParseError(#[from] MapTypeParseError),
    #[error("PointerTypeParseError {0:?}")]
    PointerTypeParseError(#[from] PointerTypeParseError),
    #[error("SliceTypeParseError {0:?}")]
    SliceTypeParseError(#[from] SliceTypeParseError),
}

impl FromStr for Type {
    type Err = TypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .map_err(|err| TypeParseError::TreeSitterLanguageError(err.to_string()))?;

        let code = format!("var _ {};", s);

        let tree = parser
            .parse(&code, None)
            .ok_or_else(|| TypeParseError::TreeSitterParseFailed("Not found tree".to_string()))?;
        let mut tree_cursor = tree.walk();
        let source = code.as_bytes();
        let node_source_file = tree.root_node();

        let node_var_declaration = node_source_file
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_declaration")
            .ok_or_else(|| {
                TypeParseError::TreeSitterParseFailed("Not found var_declaration".to_string())
            })?;
        let node_var_spec = node_var_declaration
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_spec")
            .ok_or_else(|| {
                TypeParseError::TreeSitterParseFailed("Not found var_spec".to_string())
            })?;

        let _ = node_var_spec.named_child(0).ok_or_else(|| {
            TypeParseError::TreeSitterParseFailed("Not found var_spec name".to_string())
        })?;
        let node_var_spec_type = node_var_spec.named_child(1).ok_or_else(|| {
            TypeParseError::TreeSitterParseFailed("Not found var_spec type".to_string())
        })?;

        Self::from_var_spec_type_node(node_var_spec_type, source)
    }
}

impl Type {
    pub(crate) fn from_var_spec_type_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeParseError> {
        match node.kind() {
            "qualified_type" => TypeName::from_qualified_type_node(node, source)
                .map(Self::TypeName)
                .map_err(Into::into),
            "type_identifier" => TypeName::from_type_identifier_node(node, source)
                .map(Self::TypeName)
                .map_err(Into::into),
            "array_type" => ArrayType::from_array_type_node(node, source).map(Self::ArrayType),
            "map_type" => MapType::from_map_type_node(node, source).map(Self::MapType),
            "pointer_type" => {
                PointerType::from_pointer_type_node(node, source).map(Self::PointerType)
            }
            "slice_type" => SliceType::from_slice_type_node(node, source).map(Self::SliceType),
            _ => Err(TypeParseError::UnsupportedType(node.kind().to_owned())),
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::TypeName(type_name) => tokens.append_all(quote!(#type_name)),
            Type::ArrayType(array_type) => tokens.append_all(quote!(#array_type)),
            Type::PointerType(pointer_type) => tokens.append_all(quote!(#pointer_type)),
            Type::SliceType(slice_type) => tokens.append_all(quote!(#slice_type)),
            Type::MapType(map_type) => tokens.append_all(quote!(#map_type)),
        }
    }
}

pub use golang_struct_tag;
pub use golang_type_name::{self, TypeName, TypeNameParseError};

use std::str::{self, FromStr};

use tree_sitter::Node;

pub mod array_type;
pub mod channel_type;
pub mod function_type;
pub mod interface_type;
pub mod map_type;
pub mod parenthesized_type;
pub mod pointer_type;
pub mod slice_type;
pub mod struct_type;

pub use self::array_type::{ArrayLength, ArrayType, ArrayTypeParseError};
pub use self::channel_type::{ChannelType, ChannelTypeParseError};
pub use self::function_type::{FunctionType, FunctionTypeParseError};
pub use self::interface_type::{InterfaceType, InterfaceTypeParseError};
pub use self::map_type::{MapType, MapTypeParseError};
pub use self::parenthesized_type::{ParenthesizedType, ParenthesizedTypeParseError};
pub use self::pointer_type::{PointerType, PointerTypeParseError};
pub use self::slice_type::{SliceType, SliceTypeParseError};
pub use self::struct_type::{StructField, StructType, StructTypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    TypeName(TypeName),
    //
    ArrayType(ArrayType),
    StructType(StructType),
    PointerType(PointerType),
    FunctionType(FunctionType),
    InterfaceType(InterfaceType),
    SliceType(SliceType),
    MapType(MapType),
    ChannelType(ChannelType),
    //
    ParenthesizedType(ParenthesizedType),
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
    //
    #[error("TypeNameParseError {0:?}")]
    TypeNameParseError(#[from] TypeNameParseError),
    //
    #[error("ArrayTypeParseError {0:?}")]
    ArrayTypeParseError(#[from] ArrayTypeParseError),
    #[error("StructTypeParseError {0:?}")]
    StructTypeParseError(#[from] StructTypeParseError),
    #[error("PointerTypeParseError {0:?}")]
    PointerTypeParseError(#[from] PointerTypeParseError),
    #[error("FunctionTypeParseError {0:?}")]
    FunctionTypeParseError(#[from] FunctionTypeParseError),
    #[error("InterfaceTypeParseError {0:?}")]
    InterfaceTypeParseError(#[from] InterfaceTypeParseError),
    #[error("SliceTypeParseError {0:?}")]
    SliceTypeParseError(#[from] SliceTypeParseError),
    #[error("MapTypeParseError {0:?}")]
    MapTypeParseError(#[from] MapTypeParseError),
    #[error("ChannelTypeParseError {0:?}")]
    ChannelTypeParseError(#[from] ChannelTypeParseError),
    //
    #[error("ParenthesizedTypeParseError {0:?}")]
    ParenthesizedTypeParseError(#[from] ParenthesizedTypeParseError),
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

        Self::from_node(node_var_spec_type, source)
    }
}

impl Type {
    pub fn from_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        match node.kind() {
            //
            "qualified_type" => TypeName::from_qualified_type_node(node, source)
                .map(Self::TypeName)
                .map_err(Into::into),
            "type_identifier" => TypeName::from_type_identifier_node(node, source)
                .map(Self::TypeName)
                .map_err(Into::into),
            //
            "array_type" => ArrayType::from_array_type_node(node, source).map(Self::ArrayType),
            "struct_type" => StructType::from_struct_type_node(node, source).map(Self::StructType),
            "pointer_type" => {
                PointerType::from_pointer_type_node(node, source).map(Self::PointerType)
            }
            "function_type" => {
                FunctionType::from_function_type_node(node, source).map(Self::FunctionType)
            }
            "interface_type" => {
                InterfaceType::from_interface_type_node(node, source).map(Self::InterfaceType)
            }
            "slice_type" => SliceType::from_slice_type_node(node, source).map(Self::SliceType),
            "map_type" => MapType::from_map_type_node(node, source).map(Self::MapType),
            "channel_type" => {
                ChannelType::from_channel_type_node(node, source).map(Self::ChannelType)
            }
            //
            "parenthesized_type" => ParenthesizedType::from_parenthesized_type_node(node, source)
                .map(Self::ParenthesizedType),
            _ => Err(TypeParseError::UnsupportedType(node.kind().to_owned())),
        }
    }
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::Type;

    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for Type {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                //
                Self::TypeName(type_name) => tokens.append_all(quote!(#type_name)),
                //
                Self::ArrayType(array_type) => tokens.append_all(quote!(#array_type)),
                Self::StructType(_) => {
                    let err = "impl ToTokens for StructType is unsupported";
                    tokens.append_all(quote!(compile_error!(#err)))
                }
                Self::PointerType(pointer_type) => tokens.append_all(quote!(#pointer_type)),
                Self::FunctionType(_) => {
                    let err = "impl ToTokens for FunctionType is unsupported";
                    tokens.append_all(quote!(compile_error!(#err)))
                }
                Self::InterfaceType(_) => {
                    let err = "impl ToTokens for InterfaceType is unsupported";
                    tokens.append_all(quote!(compile_error!(#err)))
                }
                Self::SliceType(slice_type) => tokens.append_all(quote!(#slice_type)),
                Self::MapType(map_type) => tokens.append_all(quote!(#map_type)),
                Self::ChannelType(_) => {
                    let err = "impl ToTokens for ChannelType is unsupported";
                    tokens.append_all(quote!(compile_error!(#err)))
                }
                //
                Self::ParenthesizedType(parenthesized_type) => {
                    tokens.append_all(quote!(#parenthesized_type))
                }
            }
        }
    }
}

use std::str;

use golang_parser::tree_sitter::Node;
use golang_type_core::{StructType, Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub r#type: Type,
}

#[derive(thiserror::Error, Debug)]
pub enum TypeDefParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(&'static str),
    #[error("Utf8Error {0:?}")]
    Utf8Error(#[from] str::Utf8Error),
    #[error("TypeParseError {0:?}")]
    TypeParseError(#[from] TypeParseError),
}

impl TypeDef {
    pub(crate) fn from_type_spec_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeDefParseError> {
        debug_assert!(node.kind() == "type_spec");

        let node_name = node
            .named_child(0)
            .ok_or(TypeDefParseError::NodeMissing("name"))?;
        let name = node_name.utf8_text(source)?;

        let node_type = node
            .named_child(1)
            .ok_or(TypeDefParseError::NodeMissing("type"))?;
        let r#type = Type::from_node(node_type, source)?;

        Ok(Self {
            name: name.to_owned(),
            r#type,
        })
    }
}

pub struct JsonStructDef {
    pub name: String,
    pub struct_type: StructType,
}

#[cfg(feature = "enable-quote-to_tokens")]
pub mod enable_quote_to_tokens {
    use super::JsonStructDef;

    use convert_case::{Case, Casing as _};
    use golang_type_core::StructField;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for JsonStructDef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let struct_name = format_ident!("{}", self.name.to_case(Case::Snake));
            let struct_fields: Vec<_> = self
                .struct_type
                .field_decls
                .iter()
                .map(|field_decl| match &field_decl.struct_field {
                    StructField::IdentifierListType(names, r#type) => names
                        .iter()
                        .map(|name| {
                            let field_name_serde_rename = name;
                            let field_name = format_ident!("{}", name.to_case(Case::Snake));
                            let field_type = r#type;

                            quote! {
                                #[serde(rename = #field_name_serde_rename)]
                                pub #field_name: #field_type,
                            }
                        })
                        .collect(),
                    StructField::EmbeddedField(embedded_field) => {
                        let field_name_serde_rename = &embedded_field.name();
                        let field_name =
                            format_ident!("{}", embedded_field.name().to_case(Case::Snake));
                        let field_type = embedded_field.r#type();

                        vec![quote! {
                            #[serde(rename = #field_name_serde_rename)]
                            pub #field_name: #field_type,
                        }]
                    }
                })
                .flatten()
                .collect();

            let token = quote! {
                #[derive(::serde::Deserialize, Debug, Clone)]
                pub struct #struct_name {
                    #(#struct_fields)*
                }
            };

            tokens.append_all(token);
        }
    }
}

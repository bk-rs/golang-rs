use std::str;

#[cfg(feature = "enable-quote-to_tokens")]
use std::collections::HashMap;

use golang_parser::tree_sitter::Node;
use golang_type_core::{Type, TypeParseError};

#[cfg(feature = "enable-quote-to_tokens")]
use golang_type_core::StructType;
#[cfg(feature = "enable-quote-to_tokens")]
use proc_macro2::TokenStream;

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

#[cfg(feature = "enable-quote-to_tokens")]
pub struct JsonStructDef {
    pub name: String,
    pub struct_type: StructType,
    pub field_opts: HashMap<JsonStructFieldName, JsonStructFieldOption>,
}

#[cfg(feature = "enable-quote-to_tokens")]
pub type JsonStructFieldName = String;

#[cfg(feature = "enable-quote-to_tokens")]
#[derive(Default, Debug, Clone)]
pub struct JsonStructFieldOption {
    pub r#type: Option<TokenStream>,
    pub serde_deserialize_with: Option<String>,
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::JsonStructDef;

    use convert_case::{Case, Casing as _};
    use golang_type_core::{
        golang_struct_tag::{JsonStructTag, JsonStructTagOption},
        StructField, Type, TypeName,
    };
    use proc_macro2::{Punct, Spacing, TokenStream};
    use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for JsonStructDef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let struct_name = format_ident!("{}", self.name.to_case(Case::Pascal));
            let struct_fields: Vec<_> = self
                .struct_type
                .field_decls
                .iter()
                .map(|field_decl| {
                    let as_json_struct_tag = if let Some(tag) = &field_decl.tag {
                        tag.as_json_struct_tag()
                    } else {
                        None
                    };

                    let is_ignored = if let Some(JsonStructTag::Ignored) = as_json_struct_tag {
                        Some(true)
                    } else {
                        None
                    };
                    if is_ignored == Some(true) {
                        return vec![];
                    }

                    let rename = if let Some(JsonStructTag::Normal(rename, _)) = as_json_struct_tag
                    {
                        rename.to_owned()
                    } else {
                        None
                    };
                    let is_omitempty =
                        if let Some(JsonStructTag::Normal(_, options)) = as_json_struct_tag {
                            Some(options.contains(&JsonStructTagOption::Omitempty))
                        } else {
                            None
                        };
                    let is_string =
                        if let Some(JsonStructTag::Normal(_, options)) = as_json_struct_tag {
                            Some(options.contains(&JsonStructTagOption::String))
                        } else {
                            None
                        };

                    match &field_decl.struct_field {
                        StructField::IdentifierListType(names, r#type) => names
                            .iter()
                            .filter(|x| x != &"_")
                            .map(|name| {
                                let field_opt = self
                                    .field_opts
                                    .get(name)
                                    .map(ToOwned::to_owned)
                                    .unwrap_or_default();

                                let field_serde_attr = JsonStructFieldSerdeAttr {
                                    rename: rename.to_owned().unwrap_or_else(|| name.to_owned()),
                                    is_omitempty,
                                    serde_deserialize_with: field_opt.serde_deserialize_with,
                                };
                                let field_name = format_ident!("r#{}", name.to_case(Case::Snake));
                                let field_type = JsonStructFieldType {
                                    r#type: *r#type.to_owned(),
                                    is_string,
                                    is_omitempty,
                                    special_type: field_opt.r#type,
                                };

                                quote! {
                                    #[serde(#field_serde_attr)]
                                    pub #field_name: #field_type,
                                }
                            })
                            .collect(),
                        StructField::EmbeddedField(embedded_field) => {
                            let name = embedded_field.name();

                            let field_opt = self
                                .field_opts
                                .get(&name)
                                .map(ToOwned::to_owned)
                                .unwrap_or_default();

                            let field_serde_attr = JsonStructFieldSerdeAttr {
                                rename: rename.unwrap_or_else(|| name.to_owned()),
                                is_omitempty,
                                serde_deserialize_with: field_opt.serde_deserialize_with,
                            };
                            let field_name = format_ident!("r#{}", name.to_case(Case::Snake));
                            let field_type = JsonStructFieldType {
                                r#type: embedded_field.r#type(),
                                is_string,
                                is_omitempty,
                                special_type: field_opt.r#type,
                            };

                            vec![quote! {
                                #[serde(#field_serde_attr)]
                                pub #field_name: #field_type,
                            }]
                        }
                    }
                })
                .flatten()
                .collect();

            let token = quote! {
                #[derive(::serde::Deserialize, ::serde::Serialize, Debug, Clone)]
                pub struct #struct_name {
                    #(#struct_fields)*
                }
            };

            tokens.append_all(token);
        }
    }

    struct JsonStructFieldSerdeAttr {
        rename: String,
        is_omitempty: Option<bool>,
        serde_deserialize_with: Option<String>,
    }
    impl ToTokens for JsonStructFieldSerdeAttr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append(format_ident!("rename"));
            tokens.append(Punct::new('=', Spacing::Alone));
            let rename = &self.rename;
            tokens.append_all(quote!(#rename));

            if self.is_omitempty == Some(true) {
                tokens.append(Punct::new(',', Spacing::Alone));

                tokens.append(format_ident!("default"));

                tokens.append(Punct::new(',', Spacing::Alone));

                tokens.append(format_ident!("skip_serializing_if"));
                tokens.append(Punct::new('=', Spacing::Alone));
                let skip_serializing_if_val = "Option::is_none";
                tokens.append_all(quote!(#skip_serializing_if_val));
            }

            if let Some(serde_deserialize_with) = &self.serde_deserialize_with {
                tokens.append(Punct::new(',', Spacing::Alone));

                tokens.append(format_ident!("deserialize_with"));
                tokens.append(Punct::new('=', Spacing::Alone));
                tokens.append_all(quote!(#serde_deserialize_with));
            }
        }
    }

    struct JsonStructFieldType {
        r#type: Type,
        is_string: Option<bool>,
        is_omitempty: Option<bool>,
        special_type: Option<TokenStream>,
    }
    impl ToTokens for JsonStructFieldType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if let Some(special_type) = &self.special_type {
                tokens.append_all(special_type.to_owned());
                return;
            }

            let r#type = &self.r#type;
            let mut token = quote!(#r#type);
            if self.is_string == Some(true) {
                let r#type = Type::TypeName(TypeName::String);
                token = quote!(#r#type);
            }
            if self.is_omitempty == Some(true) {
                let mut tokens_tmp = TokenStream::new();
                tokens_tmp.append_all(quote!(::core::option::Option));
                tokens_tmp.append(Punct::new('<', Spacing::Alone));
                tokens_tmp.append_all(token);
                tokens_tmp.append(Punct::new('>', Spacing::Alone));
                token = tokens_tmp;
            }

            tokens.append_all(token);
        }
    }
}

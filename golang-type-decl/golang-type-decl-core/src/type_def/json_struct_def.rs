use std::collections::HashMap;

use convert_case::{Case, Casing as _};
use golang_type_core::{
    golang_struct_tag::{JsonStructTag, JsonStructTagOption},
    StructField, StructType, Type, TypeName,
};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

pub struct JsonStructDef {
    pub name: String,
    pub struct_type: StructType,
    pub opt: JsonStructOption,
    pub field_opts: HashMap<JsonStructFieldName, JsonStructFieldOption>,
}

#[derive(Default, Debug)]
pub struct JsonStructOption {
    pub enable_derive_serde_ser: bool,
    pub enable_derive_serde_de: bool,
    pub enable_derive_debug: bool,
    pub enable_derive_clone: bool,
}
impl JsonStructOption {
    fn has_derive(&self) -> bool {
        self.enable_derive_serde_ser
            || self.enable_derive_serde_de
            || self.enable_derive_debug
            || self.enable_derive_clone
    }

    fn has_serde_derive(&self) -> bool {
        self.enable_derive_serde_ser || self.enable_derive_serde_de
    }
}

pub type JsonStructFieldName = String;

#[derive(Default, Debug, Clone)]
pub struct JsonStructFieldOption {
    pub special_type: Option<TokenStream>,
    pub attr_serde_deserialize_with: Option<String>,
}

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

                let rename = if let Some(JsonStructTag::Normal(rename, _)) = as_json_struct_tag {
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
                let is_string = if let Some(JsonStructTag::Normal(_, options)) = as_json_struct_tag
                {
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

                            let field_name = format_ident!("r#{}", name.to_case(Case::Snake));
                            let field_type = JsonStructFieldType {
                                r#type: *r#type.to_owned(),
                                is_string,
                                is_omitempty,
                                special_type: field_opt.special_type,
                            };

                            if self.opt.has_serde_derive() {
                                let field_serde_attr = JsonStructFieldSerdeAttr {
                                    rename: rename.to_owned().unwrap_or_else(|| name.to_owned()),
                                    is_omitempty,
                                    attr_serde_deserialize_with: field_opt
                                        .attr_serde_deserialize_with,
                                    enable_serde_ser: self.opt.enable_derive_serde_ser,
                                    enable_serde_de: self.opt.enable_derive_serde_de,
                                };

                                quote! {
                                    #[serde(#field_serde_attr)]
                                    pub #field_name: #field_type,
                                }
                            } else {
                                quote! {
                                    pub #field_name: #field_type,
                                }
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

                        let field_name = format_ident!("r#{}", name.to_case(Case::Snake));
                        let field_type = JsonStructFieldType {
                            r#type: embedded_field.r#type(),
                            is_string,
                            is_omitempty,
                            special_type: field_opt.special_type,
                        };

                        let token = if self.opt.has_serde_derive() {
                            let field_serde_attr = JsonStructFieldSerdeAttr {
                                rename: rename.unwrap_or_else(|| name.to_owned()),
                                is_omitempty,
                                attr_serde_deserialize_with: field_opt.attr_serde_deserialize_with,
                                enable_serde_ser: self.opt.enable_derive_serde_ser,
                                enable_serde_de: self.opt.enable_derive_serde_de,
                            };

                            quote! {
                                #[serde(#field_serde_attr)]
                                pub #field_name: #field_type,
                            }
                        } else {
                            quote! {
                                pub #field_name: #field_type,
                            }
                        };

                        vec![token]
                    }
                }
            })
            .flatten()
            .collect();

        let token = if self.opt.has_derive() {
            let derive_attr = JsonStructSerdeDeriveAttr {
                enable_serde_ser: self.opt.enable_derive_serde_ser,
                enable_serde_de: self.opt.enable_derive_serde_de,
                enable_debug: self.opt.enable_derive_debug,
                enable_clone: self.opt.enable_derive_clone,
            };

            quote! {
                #[derive(#derive_attr)]
                pub struct #struct_name {
                    #(#struct_fields)*
                }
            }
        } else {
            quote! {
                pub struct #struct_name {
                    #(#struct_fields)*
                }
            }
        };

        tokens.append_all(token);
    }
}

struct JsonStructSerdeDeriveAttr {
    enable_serde_ser: bool,
    enable_serde_de: bool,
    enable_debug: bool,
    enable_clone: bool,
}
impl ToTokens for JsonStructSerdeDeriveAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.enable_serde_de {
            tokens.append_all(quote!(::serde::Deserialize));
            tokens.append(Punct::new(',', Spacing::Alone));
        }

        if self.enable_serde_ser {
            tokens.append_all(quote!(::serde::Serialize));
            tokens.append(Punct::new(',', Spacing::Alone));
        }

        if self.enable_debug {
            tokens.append_all(quote!(Debug));
            tokens.append(Punct::new(',', Spacing::Alone));
        }

        if self.enable_clone {
            tokens.append_all(quote!(Clone));
        }
    }
}

struct JsonStructFieldSerdeAttr {
    rename: String,
    is_omitempty: Option<bool>,
    attr_serde_deserialize_with: Option<String>,
    enable_serde_ser: bool,
    enable_serde_de: bool,
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

            if self.enable_serde_ser {
                tokens.append(Punct::new(',', Spacing::Alone));

                tokens.append(format_ident!("skip_serializing_if"));
                tokens.append(Punct::new('=', Spacing::Alone));
                let skip_serializing_if_val = "Option::is_none";
                tokens.append_all(quote!(#skip_serializing_if_val));
            }
        }

        if let Some(serde_deserialize_with) = &self.attr_serde_deserialize_with {
            if self.enable_serde_de {
                tokens.append(Punct::new(',', Spacing::Alone));

                tokens.append(format_ident!("deserialize_with"));
                tokens.append(Punct::new('=', Spacing::Alone));
                tokens.append_all(quote!(#serde_deserialize_with));
            }
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
        let mut token = if let Some(special_type) = &self.special_type {
            special_type.to_owned()
        } else {
            let r#type = &self.r#type;
            let mut token = quote!(#r#type);

            if self.is_string == Some(true) {
                let r#type = Type::TypeName(TypeName::String);
                token = quote!(#r#type);
            }

            token
        };

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

use convert_case::{Case, Casing as _};
use golang_type_decl_core::golang_type_core::{StructField, StructType};
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

pub struct Output {
    pub name: String,
    pub struct_type: StructType,
}
impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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

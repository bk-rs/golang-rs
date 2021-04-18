use convert_case::{Case, Casing as _};
use golang_type_decl_core::{golang_type_core::Type, TypeDecl, TypeSpec};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod input;

pub use self::input::Input;

pub fn get_output(input: Input) -> TokenStream {
    let type_decl = match input.code.parse::<TypeDecl>() {
        Ok(type_decl) => type_decl,
        Err(err) => {
            let err = err.to_string();
            return quote!(compile_error!(#err));
        }
    };

    let type_def = match type_decl.type_specs.first() {
        Some(TypeSpec::TypeDef(type_def)) => type_def,
        Some(TypeSpec::AliasDecl(_)) => {
            let err = "";
            return quote!(compile_error!(#err));
        }
        None => {
            let err = "";
            return quote!(compile_error!(#err));
        }
    };

    let name = &type_def.name;
    let struct_type = match &type_def.r#type {
        Type::StructType(struct_type) => struct_type,
        _ => {
            let err = "";
            return quote!(compile_error!(#err));
        }
    };

    let struct_name = format_ident!("{}", name.to_case(Case::Snake));
    let struct_fields: Vec<_> = struct_type
        .fields
        .iter()
        .map(|field| {
            let field_name_serde_rename = &field.name;
            let field_name = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_type = field.r#type.to_owned();

            quote! {
                #[serde(rename = #field_name_serde_rename)]
                pub #field_name: #field_type,
            }
        })
        .collect();

    quote! {
        #[derive(::serde::Deserialize, Debug, Clone)]
        pub struct #struct_name {
            #(#struct_fields)*
        }
    }
}

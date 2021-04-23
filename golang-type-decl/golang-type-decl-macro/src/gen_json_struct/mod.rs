use golang_type_decl_core::{golang_type_core::Type, type_def::JsonStructDef, TypeDecl, TypeSpec};
use proc_macro2::TokenStream;
use quote::quote;

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

    let type_def = match type_decl.type_specs.into_iter().nth(input.index) {
        Some(TypeSpec::TypeDef(type_def)) => type_def,
        Some(TypeSpec::AliasDecl(_)) => {
            let err = "Require [Type definitions](https://golang.org/ref/spec#TypeDef)";
            return quote!(compile_error!(#err));
        }
        None => {
            let err = "Require [Type definitions](https://golang.org/ref/spec#TypeDef)";
            return quote!(compile_error!(#err));
        }
    };

    let name = &type_def.name;
    let struct_type = match &type_def.r#type {
        Type::StructType(struct_type) => struct_type,
        _ => {
            let err =
                "Require type definition [StructType](https://golang.org/ref/spec#StructType)";
            return quote!(compile_error!(#err));
        }
    };

    let json_struct_def = JsonStructDef {
        name: name.to_owned(),
        struct_type: struct_type.to_owned(),
    };

    quote!(#json_struct_def)
}

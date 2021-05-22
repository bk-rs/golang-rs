use golang_type_decl_core::{
    golang_type_core::{StructField, Type},
    type_def::json_struct::{JsonStruct, JsonStructOption},
    TypeDecl, TypeSpec,
};
use proc_macro2::TokenStream;
use quote::quote;

mod field_opts;
mod field_types;
mod input;

pub use self::input::Input;

#[allow(clippy::needless_collect)]
pub fn get_output(input: Input) -> TokenStream {
    let type_decl = match input.code.parse::<TypeDecl>() {
        Ok(type_decl) => type_decl,
        Err(err) => {
            let err = err.to_string();
            return quote!(compile_error!(#err));
        }
    };

    let type_def = match type_decl.type_specs.into_iter().nth(input.nth) {
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

    let field_names: Vec<_> = struct_type
        .field_decls
        .iter()
        .map(|field_decl| match &field_decl.struct_field {
            StructField::IdentifierListType(names, _) => names.to_owned(),
            StructField::EmbeddedField(embedded_field) => vec![embedded_field.name()],
        })
        .flatten()
        .collect();
    for field_name in input.field_opts.0.keys() {
        if !field_names.contains(field_name) {
            let err = format!("field [{}] not found", field_name);
            return quote!(compile_error!(#err));
        }
    }

    let json_struct = JsonStruct {
        name: name.to_owned(),
        struct_type: struct_type.to_owned(),
        opt: JsonStructOption {
            enable_derive_serde_ser: !input.disable_derive_serde_ser,
            enable_derive_serde_de: !input.disable_derive_serde_de,
            custom_derive: input.custom_derive,
            alias_name: input.alias_name,
        },
        field_opts: input.field_opts.0,
    };

    quote!(#json_struct)
}

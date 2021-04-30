use golang_type_decl_core::{
    alias_decl::type_alias::{TypeAlias, TypeAliasOption},
    TypeDecl, TypeSpec,
};
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

    let alias_decl = match type_decl.type_specs.into_iter().nth(input.nth) {
        Some(TypeSpec::TypeDef(_)) => {
            let err = "Require [Alias declarations](https://golang.org/ref/spec#AliasDecl)";
            return quote!(compile_error!(#err));
        }
        Some(TypeSpec::AliasDecl(alias_decl)) => alias_decl,
        None => {
            let err = "Require [Alias declarations](https://golang.org/ref/spec#AliasDecl)";
            return quote!(compile_error!(#err));
        }
    };

    let type_alias = TypeAlias {
        name: alias_decl.name,
        r#type: alias_decl.r#type,
        opt: TypeAliasOption {
            alias_name: input.alias_name,
            special_type: input.r#type.map(|ty| quote!(#ty)),
        },
    };

    quote!(#type_alias)
}

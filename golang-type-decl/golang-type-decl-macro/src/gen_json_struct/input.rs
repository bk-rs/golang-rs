use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitBool, LitInt, LitStr, Token,
};

use crate::utils::path_to_code;

use super::{field_opts::FieldOpts, field_types::FieldTypes};

pub struct Input {
    pub code: String,
    pub nth: usize,
    //
    pub disable_derive_serde_ser: bool,
    pub disable_derive_serde_de: bool,
    pub disable_derive_debug: bool,
    pub disable_derive_clone: bool,

    pub alias_name: Option<String>,
    //
    pub field_opts: FieldOpts,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut code = String::new();
        let mut nth = 0;

        let mut disable_derive_serde_ser = false;
        let mut disable_derive_serde_de = false;
        let mut disable_derive_debug = false;
        let mut disable_derive_clone = false;

        let mut alias_name = None;

        let mut field_types = FieldTypes::default();
        let mut field_opts = FieldOpts::default();

        while !input.is_empty() {
            let key = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;

            if key == "code" {
                let s = input.parse::<LitStr>()?.value();
                input.parse::<Token![,]>()?;

                code = s.trim_start().trim_end().to_owned();
            } else if key == "path" {
                let s = input.parse::<LitStr>()?.value();
                input.parse::<Token![,]>()?;

                match path_to_code(&s) {
                    Ok(s) => code = s,
                    Err(err) => {
                        return Err(SynError::new_spanned(key, err));
                    }
                }
            } else if key == "nth" {
                nth = input.parse::<LitInt>()?.base10_parse::<usize>()?;
                input.parse::<Token![,]>()?;
            } else if key == "disable_derive_serde_ser" {
                disable_derive_serde_ser = input.parse::<LitBool>()?.value();
                input.parse::<Token![,]>()?;
            } else if key == "disable_derive_serde_de" {
                disable_derive_serde_de = input.parse::<LitBool>()?.value();
                input.parse::<Token![,]>()?;
            } else if key == "disable_derive_debug" {
                disable_derive_debug = input.parse::<LitBool>()?.value();
                input.parse::<Token![,]>()?;
            } else if key == "disable_derive_clone" {
                disable_derive_clone = input.parse::<LitBool>()?.value();
                input.parse::<Token![,]>()?;
            } else if key == "alias_name" {
                alias_name = Some(input.parse::<LitStr>()?.value());
                input.parse::<Token![,]>()?;
            } else if key == "field_types" {
                field_types = input.parse()?;
                input.parse::<Token![,]>()?;
            } else if key == "field_opts" {
                field_opts = input.parse()?;
                input.parse::<Token![,]>()?;
            } else {
                let err = format!("unexpected input key: {}", key);
                return Err(SynError::new_spanned(key, err));
            }
        }

        for (field_name, field_type) in field_types.0 {
            let field_opt = field_opts.0.entry(field_name).or_default();
            field_opt.special_type = Some(quote!(#field_type));
        }

        Ok(Self {
            code,
            nth,
            disable_derive_serde_ser,
            disable_derive_serde_de,
            disable_derive_debug,
            disable_derive_clone,
            alias_name,
            field_opts,
        })
    }
}

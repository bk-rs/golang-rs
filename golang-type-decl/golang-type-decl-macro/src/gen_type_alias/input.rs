use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitInt, LitStr, Token, Type,
};

use crate::utils::path_to_code;

pub struct Input {
    pub code: String,
    pub nth: usize,
    //
    pub alias_name: Option<String>,
    pub r#type: Option<Type>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut code = String::new();
        let mut nth = 0;

        let mut alias_name = None;
        let mut r#type = None;

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
            } else if key == "alias_name" {
                alias_name = Some(input.parse::<LitStr>()?.value());
                input.parse::<Token![,]>()?;
            } else if key == "type_" {
                r#type = Some(input.parse::<Type>()?);
                input.parse::<Token![,]>()?;
            } else {
                let err = format!("unexpected input key: {}", key);
                return Err(SynError::new_spanned(key, err));
            }
        }

        Ok(Self {
            code,
            nth,
            alias_name,
            r#type,
        })
    }
}

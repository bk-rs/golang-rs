use std::{env, fs, path::PathBuf};

use quote::quote;
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitBool, LitInt, LitStr, Token,
};
use url::Url;

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

fn path_to_code(path: &str) -> Result<String, String> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "CARGO_MANIFEST_DIR is not set; please use Cargo to build".to_owned())?;

    let mut path = PathBuf::from(cargo_manifest_dir).join(path);

    let url = Url::parse(format!("file://{}", path.to_str().unwrap()).as_str())
        .map_err(|err| format!("failed to read file at {:?}: {}", path, err))?;

    let (line_start, line_end) = if let Some(fragment) = url.fragment() {
        parse_fragment(fragment)
            .map(|v| {
                path = PathBuf::from(url.path());
                v
            })
            .map_err(|err| format!("file invalid at {:?}: {}", path, err))?
    } else {
        (None, None)
    };

    if !path.exists() {
        return Err(format!("file not exists at {:?}", path));
    }

    let content = fs::read_to_string(&path)
        .map_err(|err| format!("failed to read file at {:?}: {}", path, err))?;

    if let Some(line_start) = line_start {
        Ok(content
            .lines()
            .skip(line_start - 1)
            .take(
                if let Some(line_end) = line_end {
                    line_end - line_start
                } else {
                    0
                } + 1,
            )
            .collect::<Vec<_>>()
            .join("\r\n"))
    } else {
        Ok(content)
    }
}

fn parse_fragment(fragment: &str) -> Result<(Option<usize>, Option<usize>), String> {
    let re = Regex::new(r"^L(?P<start>[\d]+)(-L(?P<end>[\d]+))?$").unwrap();

    let cap = re
        .captures_iter(fragment)
        .next()
        .ok_or_else(|| "fragment invalid".to_owned())?;

    let start = if let Some(val) = cap.name("start") {
        Some(
            val.as_str()
                .parse::<usize>()
                .map_err(|err| err.to_string())?,
        )
    } else {
        None
    };

    let end = if let Some(val) = cap.name("end") {
        Some(
            val.as_str()
                .parse::<usize>()
                .map_err(|err| err.to_string())?,
        )
    } else {
        None
    };

    if end.is_some() && start.unwrap_or_default() > end.unwrap_or_default() {
        return Err("fragment invalid".to_owned());
    }

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fragment() {
        match parse_fragment("L1") {
            Ok((start, end)) => {
                assert_eq!(start, Some(1));
                assert_eq!(end, None);
            }
            Err(err) => assert!(false, "{}", err),
        }

        match parse_fragment("L1-L2") {
            Ok((start, end)) => {
                assert_eq!(start, Some(1));
                assert_eq!(end, Some(2));
            }
            Err(err) => assert!(false, "{}", err),
        }

        match parse_fragment("Ln") {
            Ok(_) => assert!(false),
            Err(_) => (),
        }

        match parse_fragment("L1-L2-L3") {
            Ok(_) => assert!(false),
            Err(_) => (),
        }
    }
}

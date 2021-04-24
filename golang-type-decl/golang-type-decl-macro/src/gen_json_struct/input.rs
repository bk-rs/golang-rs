use std::{env, fs, path::PathBuf};

use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitStr, Token,
};
use url::Url;

pub struct Input {
    pub code: String,
    pub index: usize,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut code = String::new();
        let index = 0;

        let mut expect_comma = false;

        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<Token![,]>()?;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "code" {
                code = input
                    .parse::<LitStr>()?
                    .value()
                    .trim_start()
                    .trim_end()
                    .to_owned();
            } else if key == "path" {
                let path = input.parse::<LitStr>()?.value();

                let cargo_manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
                    Ok(cargo_manifest_dir) => cargo_manifest_dir,
                    Err(_) => {
                        let message = "CARGO_MANIFEST_DIR is not set; please use Cargo to build";
                        return Err(SynError::new_spanned(key, message));
                    }
                };

                let mut path = PathBuf::from(cargo_manifest_dir).join(path);

                let url = match Url::parse(format!("file://{}", path.to_str().unwrap()).as_str()) {
                    Ok(url) => url,
                    Err(err) => {
                        let message = format!("failed to read file at {:?}: {}", path, err);
                        return Err(SynError::new_spanned(key, message));
                    }
                };

                let (line_start, line_end) = if let Some(fragment) = url.fragment() {
                    match parse_fragment(fragment) {
                        Ok((start, end)) => {
                            path = PathBuf::from(url.path());

                            (start, end)
                        }
                        Err(err) => {
                            let message = format!("file invalid at {:?}: {}", path, err);
                            return Err(SynError::new_spanned(key, message));
                        }
                    }
                } else {
                    (None, None)
                };

                if !path.exists() {
                    let message = format!("file not exists at {:?}", path);
                    return Err(SynError::new_spanned(key, message));
                }

                let content = match fs::read_to_string(&path) {
                    Ok(str) => str,
                    Err(err) => {
                        let message = format!("failed to read file at {:?}: {}", path, err);
                        return Err(SynError::new_spanned(key, message));
                    }
                };

                code = if let Some(line_start) = line_start {
                    content
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
                        .join("\r\n")
                } else {
                    content
                };

                println!("{}", code);
            } else {
                let message = format!("unexpected input key: {}", key);
                return Err(SynError::new_spanned(key, message));
            }

            expect_comma = true;
        }

        Ok(Self { code, index })
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

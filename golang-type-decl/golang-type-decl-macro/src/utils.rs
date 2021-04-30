use std::{env, fs, path::PathBuf};

use regex::Regex;
use url::Url;

pub(crate) fn path_to_code(path: &str) -> Result<String, String> {
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

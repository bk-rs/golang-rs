use std::{error, fs, path::PathBuf};

use golang_struct_tag::{ConventionStructTag, JsonStructTag, JsonStructTagOption, StructTag};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/json.txt"))?;
    for line in content.lines() {
        let mut split = line.split("\t");
        let name = split.next().unwrap();
        let str = split.next().unwrap();
        assert!(split.next().is_none());

        match name {
            "A" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        None,
                        vec![]
                    ))]
                    .into_iter()
                    .collect(),
                ),
                str.parse()?
            ),
            "B" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Ignored)]
                        .into_iter()
                        .collect()
                ),
                str.parse()?
            ),
            "C" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("-".to_owned()),
                        vec![]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "D" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("d".to_owned()),
                        vec![]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "E" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        None,
                        vec![JsonStructTagOption::Omitempty]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "F" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        None,
                        vec![JsonStructTagOption::String]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "G" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("g".to_owned()),
                        vec![JsonStructTagOption::Omitempty]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "H" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("h".to_owned()),
                        vec![JsonStructTagOption::String]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "I" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("i".to_owned()),
                        vec![JsonStructTagOption::Omitempty, JsonStructTagOption::String]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "J" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("j".to_owned()),
                        vec![JsonStructTagOption::String, JsonStructTagOption::Omitempty]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "K" => assert_eq!(
                StructTag::Convention(
                    vec![ConventionStructTag::Json(JsonStructTag::Normal(
                        Some("k".to_owned()),
                        vec![
                            JsonStructTagOption::Unknown("foo".to_owned()),
                            JsonStructTagOption::Unknown("bar".to_owned())
                        ]
                    ))]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            "L" => assert_eq!(
                StructTag::Convention(
                    vec![
                        ConventionStructTag::Json(JsonStructTag::Normal(
                            Some("l".to_owned()),
                            vec![]
                        )),
                        ConventionStructTag::Unknown("xml".to_owned(), "".to_owned())
                    ]
                    .into_iter()
                    .collect()
                ),
                str.parse()?
            ),
            _ => assert!(false),
        }
    }

    Ok(())
}

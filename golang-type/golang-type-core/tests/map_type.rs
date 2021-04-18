use std::{error, fs, path::PathBuf};

use golang_type_core::{golang_type_name_core::TypeName, MapType, SliceType, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/map_type.txt"))?;
    for (i, str) in content.lines().enumerate() {
        match i + 1 {
            1 => assert_eq!(
                Type::MapType(MapType {
                    key: Type::TypeName(TypeName::String).into(),
                    value: Type::TypeName(TypeName::Int).into()
                }),
                str.parse()?
            ),
            2 => {}
            3 => {}
            4 => assert_eq!(
                Type::MapType(MapType {
                    key: Type::TypeName(TypeName::String).into(),
                    value: Type::SliceType(SliceType {
                        element: Type::TypeName(TypeName::String).into()
                    })
                    .into()
                }),
                str.parse()?
            ),
            _ => {}
        }
    }

    Ok(())
}

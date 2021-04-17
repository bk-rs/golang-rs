use std::{error, fs, path::PathBuf};

use golang_type_core::{golang_type_name::TypeName, SliceType, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/slice_type.txt"))?;
    for (i, str) in content.lines().enumerate() {
        match i + 1 {
            1 => assert_eq!(
                Type::SliceType(SliceType {
                    element: Type::TypeName(TypeName::Int).into()
                }),
                str.parse()?
            ),

            _ => {}
        }
    }

    Ok(())
}

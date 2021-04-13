use std::{error, fs, path::PathBuf};

use golang_type::{golang_type_name::TypeName, ArrayLength, ArrayType, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/array_type.txt"))?;
    for (i, line) in content.lines().enumerate() {
        let mut split = line.split("\t");
        let str = split.next().unwrap();
        assert!(split.next().is_none());

        match i + 1 {
            1 => assert_eq!(
                Type::ArrayType(ArrayType {
                    length: ArrayLength::IntLiteral(32),
                    element: Type::TypeName(TypeName::Uint8).into()
                }),
                str.parse()?
            ),

            _ => {}
        }
    }

    Ok(())
}

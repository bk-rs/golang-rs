use std::{error, fs, path::PathBuf};

use golang_type_core::{golang_type_name::TypeName, ArrayLength, ArrayType, PointerType, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/array_type.txt"))?;
    for (i, str) in content.lines().enumerate() {
        match i + 1 {
            1 => assert_eq!(
                Type::ArrayType(ArrayType {
                    length: ArrayLength::IntLiteral(32),
                    element: Type::TypeName(TypeName::Byte).into()
                }),
                str.parse()?
            ),
            2 => {}
            3 => assert_eq!(
                Type::ArrayType(ArrayType {
                    length: ArrayLength::IntLiteral(1000),
                    element: Type::PointerType(PointerType(
                        Type::TypeName(TypeName::Float64).into()
                    ))
                    .into()
                }),
                str.parse()?
            ),
            4 => assert_eq!(
                Type::ArrayType(ArrayType {
                    length: ArrayLength::IntLiteral(3),
                    element: Type::ArrayType(ArrayType {
                        length: ArrayLength::IntLiteral(5),
                        element: Type::TypeName(TypeName::Int).into()
                    })
                    .into()
                }),
                str.parse()?
            ),
            5 => assert_eq!(
                Type::ArrayType(ArrayType {
                    length: ArrayLength::IntLiteral(2),
                    element: Type::ArrayType(ArrayType {
                        length: ArrayLength::IntLiteral(2),
                        element: Type::ArrayType(ArrayType {
                            length: ArrayLength::IntLiteral(2),
                            element: Type::TypeName(TypeName::Float64).into()
                        })
                        .into()
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

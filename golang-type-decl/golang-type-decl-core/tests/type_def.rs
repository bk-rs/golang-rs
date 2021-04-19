use std::{error, fs, path::PathBuf};

use golang_type_decl_core::{
    golang_type_core::{FieldDecl, PointerType, StructField, StructType, Type, TypeName},
    TypeDecl, TypeDef, TypeSpec,
};

#[test]
fn test_parse_single() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/simple.go"))?;
    let str = content
        .lines()
        .skip(15)
        .take(4)
        .collect::<Vec<_>>()
        .join("\r\n");

    assert_eq!(
        TypeDecl {
            type_specs: vec![TypeSpec::TypeDef(TypeDef {
                name: "TreeNode".to_owned(),
                r#type: Type::StructType(StructType {
                    field_decls: vec![
                        FieldDecl {
                            struct_field: StructField::IdentifierListType(
                                vec!["left".to_owned(), "right".to_owned()],
                                Type::PointerType(PointerType(
                                    Type::TypeName(TypeName::Identifier("TreeNode".to_owned()))
                                        .into()
                                ))
                                .into()
                            ),
                            tag: None,
                        },
                        FieldDecl {
                            struct_field: StructField::IdentifierListType(
                                vec!["value".to_owned()],
                                Type::PointerType(PointerType(
                                    Type::TypeName(TypeName::Identifier("Comparable".to_owned()))
                                        .into()
                                ))
                                .into()
                            ),
                            tag: None,
                        },
                    ]
                })
            })]
        },
        str.parse()?
    );

    Ok(())
}

#[test]
fn test_parse_multi() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/simple.go"))?;
    let str = content
        .lines()
        .skip(10)
        .take(4)
        .collect::<Vec<_>>()
        .join("\r\n");

    assert_eq!(
        TypeDecl {
            type_specs: vec![
                TypeSpec::TypeDef(TypeDef {
                    name: "Point".to_owned(),
                    r#type: Type::StructType(StructType {
                        field_decls: vec![FieldDecl {
                            struct_field: StructField::IdentifierListType(
                                vec!["x".to_owned(), "y".to_owned()],
                                Type::TypeName(TypeName::Float64).into(),
                            ),
                            tag: None,
                        },]
                    })
                }),
                TypeSpec::TypeDef(TypeDef {
                    name: "polar".to_owned(),
                    r#type: Type::TypeName(TypeName::Identifier("Point".to_owned())),
                })
            ]
        },
        str.parse()?
    );

    Ok(())
}

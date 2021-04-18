use std::{error, fs, path::PathBuf};

use golang_type_decl_core::{
    golang_type_core::{PointerType, StructField, StructType, Type, TypeName},
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
                    fields: vec![
                        StructField {
                            name: "left".to_owned(),
                            r#type: Type::PointerType(PointerType(
                                Type::TypeName(TypeName::Identifier("TreeNode".to_owned())).into()
                            ))
                            .into(),
                            is_embedded: false,
                            tag: None,
                        },
                        StructField {
                            name: "right".to_owned(),
                            r#type: Type::PointerType(PointerType(
                                Type::TypeName(TypeName::Identifier("TreeNode".to_owned())).into()
                            ))
                            .into(),
                            is_embedded: false,
                            tag: None,
                        },
                        StructField {
                            name: "value".to_owned(),
                            r#type: Type::PointerType(PointerType(
                                Type::TypeName(TypeName::Identifier("Comparable".to_owned()))
                                    .into()
                            ))
                            .into(),
                            is_embedded: false,
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
                        fields: vec![
                            StructField {
                                name: "x".to_owned(),
                                r#type: Type::TypeName(TypeName::Float64).into(),
                                is_embedded: false,
                                tag: None,
                            },
                            StructField {
                                name: "y".to_owned(),
                                r#type: Type::TypeName(TypeName::Float64).into(),
                                is_embedded: false,
                                tag: None,
                            },
                        ]
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

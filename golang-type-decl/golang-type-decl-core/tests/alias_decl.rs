use std::{error, fs, path::PathBuf};

use golang_type_decl_core::{
    golang_type_core::{PointerType, SliceType, Type, TypeName},
    AliasDecl, TypeDecl, TypeSpec,
};

#[test]
fn test_parse_single() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/simple.go"))?;
    let str = content.lines().skip(2).next().unwrap();

    assert_eq!(
        TypeDecl {
            type_specs: vec![TypeSpec::AliasDecl(AliasDecl {
                name: "Node".to_owned(),
                r#type: Type::TypeName(TypeName::Int)
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
        .skip(5)
        .take(4)
        .collect::<Vec<_>>()
        .join("\r\n");

    assert_eq!(
        TypeDecl {
            type_specs: vec![
                TypeSpec::AliasDecl(AliasDecl {
                    name: "nodeList".to_owned(),
                    r#type: Type::SliceType(SliceType {
                        element: Type::PointerType(PointerType(
                            Type::TypeName(TypeName::Identifier("Node".to_owned())).into()
                        ))
                        .into()
                    })
                }),
                TypeSpec::AliasDecl(AliasDecl {
                    name: "Polar".to_owned(),
                    r#type: Type::TypeName(TypeName::Identifier("polar".to_owned()))
                }),
            ]
        },
        str.parse()?
    );

    Ok(())
}

use std::{error, fs, path::PathBuf};

use golang_type_core::{
    golang_struct_tag::{ConventionStructTag, StructTag},
    golang_type_name_core::TypeName,
    FunctionType, PointerType, SliceType, StructField, StructType, StructTypeParseError, Type,
    TypeParseError,
};

#[test]
fn test_parse_embedded_field() -> Result<(), Box<dyn error::Error>> {
    let content =
        fs::read_to_string(PathBuf::new().join("tests/files/struct_type/embedded_field.txt"))?;

    let r#type: Type = content.parse()?;

    assert_eq!(
        r#type,
        Type::StructType(StructType {
            fields: vec![
                StructField {
                    name: "T1".to_owned(),
                    r#type: Type::TypeName(TypeName::Identifier("T1".to_owned()).to_owned()).into(),
                    is_embedded: true,
                    tag: None,
                },
                StructField {
                    name: "T2".to_owned(),
                    r#type: Type::TypeName(TypeName::Identifier("T2".to_owned()).to_owned()).into(),
                    is_embedded: true,
                    tag: None,
                },
                StructField {
                    name: "Duration".to_owned(),
                    r#type: Type::TypeName(
                        TypeName::QualifiedIdent("P".to_owned(), "Duration".to_owned()).to_owned()
                    )
                    .into(),
                    is_embedded: true,
                    tag: None,
                },
                StructField {
                    name: "Month".to_owned(),
                    r#type: Type::TypeName(
                        TypeName::QualifiedIdent("P".to_owned(), "Month".to_owned()).to_owned()
                    )
                    .into(),
                    is_embedded: true,
                    tag: None,
                },
                StructField {
                    name: "x".to_owned(),
                    r#type: Type::TypeName(TypeName::Int).into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "y".to_owned(),
                    r#type: Type::TypeName(TypeName::Int).into(),
                    is_embedded: false,
                    tag: None,
                },
            ]
        })
    );

    Ok(())
}

#[test]
fn test_parse_normal() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/struct_type/normal.txt"))?;

    let r#type: Type = content.parse()?;

    assert_eq!(
        r#type,
        Type::StructType(StructType {
            fields: vec![
                StructField {
                    name: "x".to_owned(),
                    r#type: Type::TypeName(TypeName::Int).into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "y".to_owned(),
                    r#type: Type::TypeName(TypeName::Int).into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "u".to_owned(),
                    r#type: Type::TypeName(TypeName::Float32).into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "_".to_owned(),
                    r#type: Type::TypeName(TypeName::Float32).into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "A".to_owned(),
                    r#type: Type::PointerType(
                        PointerType(
                            Type::SliceType(SliceType {
                                element: Type::TypeName(TypeName::Int).into()
                            })
                            .into()
                        )
                        .into()
                    )
                    .into(),
                    is_embedded: false,
                    tag: None,
                },
                StructField {
                    name: "F".to_owned(),
                    r#type: Type::FunctionType(FunctionType {}).into(),
                    is_embedded: false,
                    tag: None,
                },
            ]
        })
    );

    Ok(())
}

#[test]
fn test_parse_tag() -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(PathBuf::new().join("tests/files/struct_type/tag.txt"))?;

    let r#type: Type = content.parse()?;

    assert_eq!(
        r#type,
        Type::StructType(StructType {
            fields: vec![
                StructField {
                    name: "microsec".to_owned(),
                    r#type: Type::TypeName(TypeName::Uint64).into(),
                    is_embedded: false,
                    tag: Some(StructTag::Convention(vec![ConventionStructTag::Unknown(
                        "protobuf".to_owned(),
                        "1".to_owned()
                    )])),
                },
                StructField {
                    name: "serverIP6".to_owned(),
                    r#type: Type::TypeName(TypeName::Uint64).into(),
                    is_embedded: false,
                    tag: Some(StructTag::Convention(vec![ConventionStructTag::Unknown(
                        "protobuf".to_owned(),
                        "2".to_owned()
                    )])),
                },
            ]
        })
    );

    Ok(())
}

// TODO
// #[test]
// fn test_parse_with_unexpected_type() -> Result<(), Box<dyn error::Error>> {
//     match r#"
//     struct {
// 		map[string]int
// 	}
//     "#
//     .parse::<Type>()
//     {
//         Ok(_) => assert!(false),
//         Err(TypeParseError::StructTypeParseError(StructTypeParseError::UnexpectedType(
//             ref err,
//         ))) if err.starts_with("unexpected type ") => {}
//         Err(err) => assert!(false, "{:?}", err),
//     }

//     Ok(())
// }

#[test]
fn test_parse_with_duplicate_field() -> Result<(), Box<dyn error::Error>> {
    match r#"
    struct {
		int
		int
	}
    "#
    .parse::<Type>()
    {
        Ok(_) => assert!(false),
        Err(TypeParseError::StructTypeParseError(StructTypeParseError::DuplicateField(
            ref err,
        ))) if err == "duplicate field int" => {}
        Err(err) => assert!(false, "{:?}", err),
    }

    match r#"
    struct {
		a int
		a uint
	}
    "#
    .parse::<Type>()
    {
        Ok(_) => assert!(false),
        Err(TypeParseError::StructTypeParseError(StructTypeParseError::DuplicateField(
            ref err,
        ))) if err == "duplicate field a" => {}
        Err(err) => assert!(false, "{}", err),
    }

    Ok(())
}

use std::error;

use golang_type_core::{golang_type_name_core::TypeName, ParenthesizedType, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn error::Error>> {
    assert_eq!(
        Type::ParenthesizedType(ParenthesizedType(Type::TypeName(TypeName::Int).into()).into()),
        "(int)".parse()?
    );

    Ok(())
}

use std::collections::HashMap;

use golang_type_decl_core::type_def::json_struct_def::JsonStructFieldName;
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, LitStr, Token, Type,
};

#[derive(Default)]
pub struct FieldTypes(pub HashMap<JsonStructFieldName, Type>);

impl Parse for FieldTypes {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut inner = HashMap::new();

        loop {
            let field_name = input.parse::<LitStr>()?;
            input.parse::<Token![=>]>()?;
            let field_type = input.parse::<Type>()?;

            if inner.insert(field_name.value(), field_type).is_some() {
                let err = format!("duplicate field name: {}", &field_name.value());
                return Err(SynError::new_spanned(field_name, err));
            }

            input.parse::<Token![,]>()?;

            if !(input.peek(LitStr) && input.peek2(Token![=>])) {
                break;
            }
        }

        Ok(Self(inner))
    }
}

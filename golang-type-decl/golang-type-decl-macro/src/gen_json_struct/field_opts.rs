use std::collections::HashMap;

use golang_type_decl_core::type_def::json_struct_def::{
    JsonStructFieldName, JsonStructFieldOption,
};
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, LitBool, LitStr, Token,
};

#[derive(Default)]
pub struct FieldOpts(pub HashMap<JsonStructFieldName, JsonStructFieldOption>);

impl Parse for FieldOpts {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut inner = HashMap::new();

        loop {
            let field_name = input.parse::<LitStr>()?;
            input.parse::<Token![=>]>()?;

            let mut field_opt = JsonStructFieldOption::default();

            loop {
                if input.peek(LitStr) && input.peek2(Token![->]) {
                    let field_opt_k = input.parse::<LitStr>()?.value();
                    input.parse::<Token![->]>()?;
                    if field_opt_k == "attr_serde_deserialize_with" {
                        let attr_serde_deserialize_with = input.parse::<LitStr>()?.value();
                        field_opt.attr_serde_deserialize_with = Some(attr_serde_deserialize_with);
                    } else if field_opt_k == "box_type" {
                        let box_type = input.parse::<LitBool>()?.value();
                        field_opt.box_type = box_type;
                    } else {
                        let err = format!("unexpected opt key: {}", field_opt_k);
                        return Err(SynError::new_spanned(field_opt_k, err));
                    }
                }

                input.parse::<Token![,]>()?;

                if !(input.peek(LitStr) && input.peek2(Token![->])) {
                    break;
                }
            }

            if inner.insert(field_name.value(), field_opt).is_some() {
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

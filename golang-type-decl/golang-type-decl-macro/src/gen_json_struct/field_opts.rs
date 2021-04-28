use std::collections::HashMap;

use golang_type_decl_core::type_def::json_struct_def::{
    JsonStructFieldName, JsonStructFieldOption,
};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, LitStr, Token, Type,
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
                    if field_opt_k == "special_type" {
                        let special_type = input.parse::<Type>()?;
                        field_opt.special_type = Some(quote!(#special_type));
                    } else if field_opt_k == "serde_deserialize_with" {
                        let serde_deserialize_with = input.parse::<LitStr>()?.value();
                        field_opt.attr_serde_deserialize_with = Some(serde_deserialize_with);
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

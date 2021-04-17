extern crate proc_macro;

use golang_type_core::{Type, TypeParseError};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error as SynError, LitStr,
};

struct TypeInput {
    r#type: Type,
}
impl Parse for TypeInput {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let r#type = input.parse::<LitStr>()?;

        let r#type: Type = r#type
            .value()
            .parse()
            .map_err(|err: TypeParseError| SynError::new_spanned(r#type, err.to_string()))?;

        Ok(Self { r#type })
    }
}

#[proc_macro]
pub fn r#type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as TypeInput);

    let r#type = input.r#type;

    let output = quote!(#r#type);
    output.into()
}

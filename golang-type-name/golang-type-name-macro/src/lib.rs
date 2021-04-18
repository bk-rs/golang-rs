extern crate proc_macro;

use golang_type_name_core::{TypeName, TypeNameParseError};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error as SynError, LitStr,
};

struct TypeNameInput {
    type_name: TypeName,
}
impl Parse for TypeNameInput {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let type_name = input.parse::<LitStr>()?;

        let type_name: TypeName = type_name
            .value()
            .parse()
            .map_err(|err: TypeNameParseError| SynError::new_spanned(type_name, err.to_string()))?;

        Ok(Self { type_name })
    }
}

#[proc_macro]
pub fn type_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as TypeNameInput);

    let type_name = input.type_name;

    let output = quote!(#type_name);
    output.into()
}

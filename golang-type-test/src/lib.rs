extern crate proc_macro;

use golang_type_core::{Type, TypeParseError};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error as SynError, Ident, LitStr, Token,
};

struct LetTypeInput {
    name: Ident,
    r#type: Type,
}
impl Parse for LetTypeInput {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![,]>()?;

        let r#type = input.parse::<LitStr>()?;

        let r#type: Type = r#type
            .value()
            .parse()
            .map_err(|err: TypeParseError| SynError::new_spanned(r#type, err.to_string()))?;

        Ok(Self { name, r#type })
    }
}

#[proc_macro]
pub fn let_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LetTypeInput);

    let name = input.name;
    let r#type = input.r#type;
    let output = quote! {
        let #name: #r#type;
    };

    output.into()
}

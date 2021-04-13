extern crate proc_macro;

use golang_type_name::{TypeName, TypeNameParseError};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error as SynError, Ident, LitStr, Token,
};

struct LetTypeNameInput {
    name: Ident,
    type_name: TypeName,
}
impl Parse for LetTypeNameInput {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![,]>()?;

        let type_name = input.parse::<LitStr>()?;

        let type_name: TypeName = type_name
            .value()
            .parse()
            .map_err(|err: TypeNameParseError| SynError::new_spanned(type_name, err.to_string()))?;

        Ok(Self { name, type_name })
    }
}

#[proc_macro]
pub fn let_type_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LetTypeNameInput);

    let name = input.name;
    let type_name = input.type_name;
    let output = quote! {
        let #name: #type_name;
    };

    output.into()
}

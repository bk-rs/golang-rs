use convert_case::{Case, Casing as _};
use golang_type_core::Type;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

pub struct TypeAlias {
    pub name: String,
    pub r#type: Type,
    pub opt: TypeAliasOption,
}

#[derive(Default, Debug)]
pub struct TypeAliasOption {
    pub alias_name: Option<String>,
    pub special_type: Option<TokenStream>,
}

impl ToTokens for TypeAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!(
            "{}",
            self.opt
                .alias_name
                .to_owned()
                .unwrap_or_else(|| self.name.to_case(Case::Pascal))
        );

        let type_token = if let Some(special_type) = &self.opt.special_type {
            special_type.to_owned()
        } else {
            let r#type = &self.r#type;
            quote!(#r#type)
        };

        let token = quote! {
            pub type #name = #type_token;
        };

        tokens.append_all(token);
    }
}

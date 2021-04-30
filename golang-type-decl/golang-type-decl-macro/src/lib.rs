extern crate proc_macro;

use syn::parse_macro_input;

mod gen_json_struct;
mod gen_type_alias;
pub(crate) mod utils;

#[proc_macro]
pub fn gen_json_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as gen_json_struct::Input);
    let output = gen_json_struct::get_output(input);
    output.into()
}

#[proc_macro]
pub fn gen_type_alias(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as gen_type_alias::Input);
    let output = gen_type_alias::get_output(input);
    output.into()
}

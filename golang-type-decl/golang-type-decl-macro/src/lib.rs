extern crate proc_macro;

use syn::parse_macro_input;

mod json_struct;

#[proc_macro]
pub fn json_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as json_struct::Input);
    let output = json_struct::get_output(input);
    output.into()
}

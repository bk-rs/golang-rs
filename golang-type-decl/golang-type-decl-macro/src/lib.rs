extern crate proc_macro;

use syn::parse_macro_input;

mod gen_json_struct;
pub(crate) mod utils;

#[proc_macro]
pub fn gen_json_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as gen_json_struct::Input);
    let output = gen_json_struct::get_output(input);
    output.into()
}

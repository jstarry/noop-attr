extern crate proc_macro;

#[proc_macro_attribute]
pub fn noop(_: proc_macro::TokenStream, body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    body
}

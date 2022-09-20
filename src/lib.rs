extern crate proc_macro;

#[allow(unused_variables)]
#[proc_macro_attribute]
pub fn noop(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[cfg(feature = "no-arguments")]
    assert!(attr.is_empty(), "this attribute takes no arguments");
    body
}

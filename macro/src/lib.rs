use proc_macro::TokenStream;
mod struct_macro;

/// Macro to implement new constructor to a struct.
#[proc_macro_derive(New)]
pub fn new_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    struct_macro::new_macro::impl_new(&ast)
}

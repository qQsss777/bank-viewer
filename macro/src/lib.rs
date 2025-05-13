use proc_macro::TokenStream;
mod struct_macro;

#[proc_macro_derive(New)]
pub fn new_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    struct_macro::new_macro::impl_new(&ast)
}

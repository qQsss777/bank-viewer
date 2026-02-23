use proc_macro::TokenStream;
use quote::quote;
/// Implements new constructor to a struct.
/// Fields are passed by value
pub fn impl_new(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("`New` derive only supports structs"),
    };

    let idents: Vec<syn::Ident> =
        data.fields.iter().filter_map(|f| f.ident.as_ref().cloned()).collect();

    let tys: Vec<&syn::Type> = data.fields.iter().map(|f| &f.ty).collect();
    let expanded = quote! {
       impl #name {
           pub fn new(#(#idents: #tys),*) -> #name {
               #name{
                #(#idents:#idents),*
            }
           }
       }
    };
    expanded.into()
}

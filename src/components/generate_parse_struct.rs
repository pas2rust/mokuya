use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_parse_struct() -> TokenStream {
    quote! {
        pub struct Parse<T>(pub T);
    }
}

use super::{generics_split_for_impl::generics_split_for_impl, get_struct_name::get_struct_name};
use quote::quote;
use syn::DeriveInput;

pub fn get_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = generics_split_for_impl(input);
    let struct_name = get_struct_name(input);

    quote! {
        #impl_generics #struct_name #type_generics #where_clause
    }
}

use syn::{DeriveInput, GenericParam};

pub fn add_traits_to_generics(input: &mut DeriveInput) {
    for param in input.generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            /*type_param
            .bounds
            .push(syn::parse_quote!(::std::default::Default));*/
            type_param.bounds.push(syn::parse_quote!(::std::fmt::Debug));
            type_param
                .bounds
                .push(syn::parse_quote!(::serde::Serialize));
            type_param
                .bounds
                .push(syn::parse_quote!(for<'de> ::serde::Deserialize<'de>));
        }
    }
}

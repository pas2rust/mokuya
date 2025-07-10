use proc_macro2::TokenStream;

pub fn for_extend_token_stream(expanded: &mut TokenStream, tokens: Vec<TokenStream>) {
    for token in tokens {
        expanded.extend::<TokenStream>(token.into());
    }
}

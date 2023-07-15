#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/unsaef/0.1.2")]

use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn ünsafe(input: TokenStream) -> TokenStream {
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new(
            &include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))[10..16],
            Span::call_site(),
        )),
        TokenTree::Group(Group::new(Delimiter::Brace, input)),
    ])
}

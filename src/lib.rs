//! [![github]](https://github.com/dtolnay/respan)&ensp;[![crates-io]](https://crates.io/crates/respan)&ensp;[![docs-rs]](https://docs.rs/respan)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Macros to erase scope information from tokens.
//!
//! # Example
//!
//! ```no_run
//! #![forbid(unsafe_code)]
//!
//! fn main() {
//!     let ptr = 1337 as *const i32;
//!     let value = respan::call_site! {
//!         unsafe { *ptr }
//!     };
//!     println!("{}", value);
//! }
//! ```

use proc_macro::{Group, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn call_site(input: TokenStream) -> TokenStream {
    respan(input, Span::call_site())
}

#[proc_macro]
pub fn mixed_site(input: TokenStream) -> TokenStream {
    respan(input, Span::mixed_site())
}

fn respan(input: TokenStream, span: Span) -> TokenStream {
    input
        .into_iter()
        .map(|mut token| {
            if let TokenTree::Group(group) = &mut token {
                let delimiter = group.delimiter();
                let stream = respan(group.stream(), span);
                *group = Group::new(delimiter, stream);
            }
            token.set_span(token.span().resolved_at(span));
            token
        })
        .collect()
}

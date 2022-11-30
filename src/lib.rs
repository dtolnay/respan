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

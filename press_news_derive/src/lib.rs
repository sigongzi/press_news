extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;

#[proc_macro_derive(PressNews, attributes(news))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    
    // Build the trait implementation
    impl_press_news(&ast)
}

fn impl_press_news(ast: &syn::DeriveInput) -> TokenStream {

    let name = &ast.ident;
    let news_part = match &ast.data {
        syn::Data::Struct(x) => {
            let s = x.fields
            .iter()
            .filter(|f| {
                // the ident must have value
                f.ident.is_some() &&
                // the news' type should be String
                f.ty.to_token_stream().to_string() == "String" &&
                // the attr for this field should have #[news]
                f.attrs.iter().filter(|a| {
                    match a.path.get_ident() {
                        Some(x) => {
                            x.to_string() == "news"
                        },
                        None => {
                            false
                        }
                    }
                }).next().is_some()
            })
            .map(|f| {
                f.ident.as_ref().unwrap()
            })
            .collect::<Vec<_>>();
            match s.len() {
                0 => {
                    panic!("there must be one field for news")
                }
                _ => {s[0]}
            }
        },
        _ => {
            panic!("can not be used for non structure here")
        }
    };
    let gen = quote! {
        impl PressNews for #name {
            fn hello(&self) {
                println!("My name is {}!", stringify!(#name));
            }
            fn press_news(&self) {
                println!("Today news: {}",self.#news_part);
            }
        }
    };
    gen.into()
}

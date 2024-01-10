extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn log_bench(_: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            println!("进入函数: {}", stringify!(#fn_name));
            #fn_block
            println!("离开函数: {} (耗时 {} ms)", stringify!(#fn_name), start.elapsed().as_millis());
        }
    };

    TokenStream::from(expanded)
}

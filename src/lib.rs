//! This proc_macro library exposes the `#[time_it]` attribute macro.
//! Annotating a function with this macro will generate a corresponding tracing event with the
//! function's execution time. By default the macro will emit DEBUG level events. This can be
//! customized by passing the desired log level as a macro argument: `#[time_it("trace")]`.
//! Works with both regular `fn`s and `async` `fn`s.
//!

use proc_macro::TokenStream;
use quote::quote;
use strum::EnumString;
use syn::{LitStr, Token, punctuated::Punctuated};

/// Attribute function used to annotate functions that should output their execution time using the
/// `tracing` library. Works with both async and non-async functions. By default, this macro will use the "DEBUG" log level.
///
/// # Example
///
/// ```rust,ignore
/// use time_it::time_it;
///
/// #[tokio::main]
/// async fn main() {
///    tracing_subscriber::fmt()
///       .with_max_level(Level::DEBUG)
///       .init();
///     test_function().await;
///     println!("Hello, world!");
/// }
///
/// #[time_it]
/// async fn test_function() {
///    println!("Some slow work");
/// }
///
/// ```
///
/// If you want to use a different log level, pass the level to to attribtue macro:
/// ```rust,ignore
/// #[time_it("trace")]
/// async fn test_function() {
///    println!("Some slow work");
/// }
/// ```
#[proc_macro_attribute]
pub fn time_it(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_attrs = &input.attrs;
    let asyncness = &input.sig.asyncness;

    let timed_fn_block = if asyncness.is_some() {
        quote! {
            let __start = tokio::time::Instant::now();
            let result = async move { #fn_block }.await;
            let __duration = __start.elapsed();
        }
    } else {
        quote! {
            let __start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let __duration = __start.elapsed();
        }
    };

    let log_level = syn::parse_macro_input!(attr as LogLevel);
    let log_line = match log_level {
        LogLevel::Trace => {
            quote! {tracing::trace!("[{}]: Execution time: {:?}", stringify!(#fn_name), __duration);}
        }
        LogLevel::Debug => {
            quote! {tracing::debug!("[{}]: Execution time: {:?}", stringify!(#fn_name), __duration);}
        }
        LogLevel::Info => {
            quote! {tracing::info!("[{}]: Execution time: {:?}", stringify!(#fn_name), __duration);}
        }
        LogLevel::Warn => {
            quote! {tracing::warn!("[{}]: Execution time: {:?}", stringify!(#fn_name), __duration);}
        }
        LogLevel::Error => {
            quote! {tracing::error!("[{}]: Execution time: {:?}", stringify!(#fn_name), __duration);}
        }
    };

    quote::quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #timed_fn_block
            #log_line
            result
        }
    }
    .into()
}

#[derive(Default, EnumString)]
#[strum(ascii_case_insensitive)]
enum LogLevel {
    Trace,
    #[default]
    Debug,
    Info,
    Warn,
    Error,
}

impl syn::parse::Parse for LogLevel {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let punctuated: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        if punctuated.len() > 1 {
            return Err(syn::Error::new(
                input.span(),
                "Unexpected multiple macro arguments",
            ));
        }
        let mut iter = punctuated.into_iter();

        let Some(first) = iter.next() else {
            return Ok(Self::default());
        };

        first
            .value()
            .parse()
            .map_err(|e| syn::Error::new(input.span(), format!("{e:?}")))
    }
}

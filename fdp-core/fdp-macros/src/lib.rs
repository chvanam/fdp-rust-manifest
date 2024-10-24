//! # FDP Definition Macros
//!
//! This module provides macros to simplify the definition of message structures and their associated metadata within the FDP system.

use fdp_common::parsing::definition::AppDefinitionModule;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

/// The `fdp::replies_with` macro is used to define a reply for a request.
/// It expects a path to the reply message as an argument.
#[proc_macro_attribute]
pub fn replies_with(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let args = parse_macro_input!(args as syn::ExprPath);
    let reply = args;
    quote! {
        #input

        impl fdp_common::mqtt::Request for #struct_name {
            type Response = #reply;
        }
    }
    .into()
}

/// The `fdp::topic` macro is used to define a message to be used within the FDP system.
/// It expects a topic string as an argument.
#[proc_macro_attribute]
pub fn topic(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if args.is_empty() {
        return TokenStream::from(quote! {
            compile_error!("A topic string is required for the message macro");
        });
    }
    let args = parse_macro_input!(args as LitStr);
    let struct_name = &input.ident;
    let topic = args.value();
    quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #input
        impl fdp_common::mqtt::Message for #struct_name {
            fn topic() -> &'static str {
                #topic
            }
        }
    }
    .into()
}

/// The `fdp::event` macro is used to define an event to be used within the FDP system.
#[proc_macro_attribute]
pub fn event(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    quote! {
        #input
        impl fdp_common::mqtt::Event for #struct_name {}
    }
    .into()
}

/// The fdp::definition macro is used to define the messages used and consumed by an application within the FDP system
#[proc_macro_attribute]
pub fn definition(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_module = parse_macro_input!(input as AppDefinitionModule);
    quote!(#input_module).into()
}

/// The `fdp::extract` macro is used to extract submodule structure, struct declarations and references from a module.
/// It flattens the top level module name (re-exporting it as `pub use module::*`) for easier access (with according `#[doc(hidden)]`).
/// It provides errors if the structure of the submodules is not as expected.
/// It can perform custom validation for any submodule's items.
/// For any given submodule, it should be able to only allow certain items, reject others and validate propretires of them.
/// It should also be able to add code to the module, such as impl blocks.
/// For a sumbodule error, it reports the errors.
/// It uses a visitor pattern to traverse the module tree.
/// It creates a `#[doc(hidden)]` get_extracted_information function that returns the extracted information.
/// The information is stored as a tree/list/ sublists of modules, where each module containts struct items corresponding
/// to items in the submodule. It could be smart to implement the parsing/visitor implementation of that struct in the same struct maybe?
/// These abstract structs for modules and items should be stored in the fdp_common crate.
#[proc_macro_attribute]
pub fn extract(_args: TokenStream, _input: TokenStream) -> TokenStream {
    todo!()
}
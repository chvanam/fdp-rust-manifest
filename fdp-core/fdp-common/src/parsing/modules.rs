//! Parsing logic for the submodules in a FDP app definition module

use crate::info::MessageReferenceInfo;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Item, ItemMod};

/// The broadcast_messages module contains message declarations
pub struct BroadcastedEventsModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for BroadcastedEventsModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let module_items = get_direct_module_items(&module);
        let gen_items = get_gen_for_declaration_module(&module_items)?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for BroadcastedEventsModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// The listened_messages module contains message references
pub struct ListenedEventsModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for ListenedEventsModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let items = get_direct_module_items(&module);
        let gen_items = get_gen_for_reference_module(&items, "broadcasted_events")?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for ListenedEventsModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// The outgoing_requests module contains message declarations
pub struct EmittedRequestsModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for EmittedRequestsModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let module_items = get_direct_module_items(&module);
        let gen_items = get_gen_for_reference_module(&module_items, "incoming_requests")?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for EmittedRequestsModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// The incoming_responses module contains message declarations
pub struct IncomingResponsesModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for IncomingResponsesModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let module_items = get_direct_module_items(&module);
        let gen_items = get_gen_for_reference_module(&module_items, "outgoing_responses")?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for IncomingResponsesModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// The incoming_requests module contains message references
pub struct IncomingRequestsModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for IncomingRequestsModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let items = get_direct_module_items(&module);
        let gen_items = get_gen_for_declaration_module(&items)?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for IncomingRequestsModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// The outgoing_responses module contains message references
pub struct OutgoingResponsesModule {
    pub module: ItemMod,
    pub gen: TokenStream,
}

impl Parse for OutgoingResponsesModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        let items = get_direct_module_items(&module);
        let gen_items = get_gen_for_declaration_module(&items)?;
        let gen = quote! { vec![#( #gen_items ),*] };
        Ok(Self { module, gen })
    }
}

impl ToTokens for OutgoingResponsesModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.module.to_tokens(tokens);
    }
}

/// Returns a list of the direct items in a module.
fn get_direct_module_items(item: &ItemMod) -> Vec<Item> {
    let input = item.clone();
    let content = if let Some((_, content)) = input.content {
        content
    } else {
        Vec::new()
    };
    content
}

/// Returns a list of the generated items for the broadcast_messages module.
fn get_gen_for_declaration_module(module_items: &Vec<Item>) -> syn::Result<Vec<TokenStream>> {
    let mut gen_items = Vec::new();
    for item in module_items {
        match item {
            Item::Struct(item_struct) => {
                let ident = &item_struct.ident;
                if let syn::Visibility::Public(_) = item_struct.vis {
                    gen_items.push(quote! {
                        MessageDeclarationInfo {
                            identifier: stringify!(#ident).to_string(),
                            topic: <#ident as fdp_common::mqtt::Message>::topic().to_string(),
                            schema: schemars::schema_for!(#ident),
                        }
                    });
                } else {
                    return Err(syn::Error::new_spanned(
                        &item_struct,
                        "A message declaration must be public.",
                    ));
                }
            }
            Item::Impl(_) => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    item,
                    "Only public struct items are allowed in this module.",
                ));
            }
        }
    }
    Ok(gen_items)
}

fn _get_gen_for_request_declaration_module(module_items: &Vec<Item>) -> syn::Result<Vec<TokenStream>> {
    let mut gen_items = Vec::new();
    for item in module_items {
        match item {
            Item::Struct(item_struct) => {
                let ident = &item_struct.ident;
                if let syn::Visibility::Public(_) = item_struct.vis {
                    gen_items.push(quote! {
                        RequestDeclarationInfo {
                            identifier: stringify!(#ident).to_string(),
                            topic: <#ident as fdp_common::mqtt::Message>::topic().to_string(),
                            schema: schemars::schema_for!(#ident),
                            response: ResponseReferenceInfo {
                                identifier: stringify!(<#ident as fdp_common::mqtt::Request>::Response),
                            }
                        }
                    });
                } else {
                    return Err(syn::Error::new_spanned(
                        &item_struct,
                        "A message declaration must be public.",
                    ));
                }
            }
            Item::Impl(_) => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    item,
                    "Only public struct items are allowed in this module.",
                ));
            }
        }
    }
    Ok(gen_items)
}

fn parse_item_use_tree(
    item: &syn::ItemUse,
    submodule_name: &str,
) -> syn::Result<MessageReferenceInfo> {
    let tree = &item.tree;
    let path = match tree {
        syn::UseTree::Path(path) if path.ident == "crate" => path,
        _ => {
            return Err(syn::Error::new_spanned(
                tree,
                "Invalid use tree structure or path does not start with 'crate::apps'",
            ))
        }
    };
    let subpath = match &*path.tree {
        syn::UseTree::Path(subpath) if subpath.ident == "apps" => subpath,
        _ => {
            return Err(syn::Error::new_spanned(
                &path.tree,
                "Path must include 'apps'",
            ))
        }
    };
    let app_path = match &*subpath.tree {
        syn::UseTree::Path(app_path) => app_path,
        _ => {
            return Err(syn::Error::new_spanned(
                &subpath.tree,
                "Expected an app name",
            ))
        }
    };
    let app_name = app_path.ident.to_string();
    let type_path = match &*app_path.tree {
        syn::UseTree::Path(type_path) => type_path,
        _ => {
            return Err(syn::Error::new_spanned(
                &app_path.tree,
                "Expected an app submodule message type in the use tree.",
            ))
        }
    };

    let message_type = type_path.ident.to_string();
    if message_type != submodule_name {
        return Err(syn::Error::new_spanned(
            &type_path.ident,
            format!(
                "Only the following submodule type '{}' is allowed here.",
                submodule_name
            ),
        ));
    }

    let message_name = match &*type_path.tree {
        syn::UseTree::Name(name) => name.ident.to_string(),
        _ => {
            return Err(syn::Error::new_spanned(
                &type_path.tree,
                "Expected a message name in the use tree.",
            ))
        }
    };

    Ok(MessageReferenceInfo {
        identifier: message_name,
        app_name,
        module: message_type,
    })
}

fn get_gen_for_reference_module(
    module_items: &Vec<Item>,
    submodule_name: &str,
) -> syn::Result<Vec<TokenStream>> {
    let mut gen_items = Vec::new();
    for item in module_items {
        match item {
            Item::Use(use_item) => {
                let MessageReferenceInfo {
                    identifier,
                    app_name,
                    module,
                } = parse_item_use_tree(&use_item, submodule_name)?;

                gen_items.push(quote! {
                    MessageReferenceInfo {
                        identifier: #identifier.to_string(),
                        app_name: #app_name.to_string(),
                        module: #module.to_string(),
                    }
                });
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    item,
                    "Only public use references are allowed in this module.",
                ));
            }
        }
    }
    Ok(gen_items)
}

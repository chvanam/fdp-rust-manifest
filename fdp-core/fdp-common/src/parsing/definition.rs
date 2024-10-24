//! Parsing logic for a FDP app definition module

use crate::parsing::modules::*;
use quote::{quote, ToTokens};
use syn::{parse::Parse, ItemMod};

/// Handles the fdp::definition macro
pub struct AppDefinitionModule {
    // Message declarations
    pub broadcasted_events: BroadcastedEventsModule,
    pub incoming_requests: IncomingRequestsModule,
    pub outgoing_responses: OutgoingResponsesModule,
    // Message references
    pub listened_events: ListenedEventsModule,
    pub emitted_requests: EmittedRequestsModule,
}

/// When parsing a FDP app definition module, we need to ensure it matches a predefined structure.
/// It can only contain 5 public sub-modules named according to the AppDefinitionModule struct.
impl Parse for AppDefinitionModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: ItemMod = input.parse()?;
        enforce_module_name(&module, "definition")?;
        let Submodules {
            broadcasted_events,
            incoming_requests,
            outgoing_responses,
            listened_events,
            emitted_requests,
        } = enforce_submodules(&module)?;

        Ok(AppDefinitionModule {
            broadcasted_events: syn::parse2(broadcasted_events.to_token_stream())?,
            incoming_requests: syn::parse2(incoming_requests.to_token_stream())?,
            outgoing_responses: syn::parse2(outgoing_responses.to_token_stream())?,
            listened_events: syn::parse2(listened_events.to_token_stream())?,
            emitted_requests: syn::parse2(emitted_requests.to_token_stream())?,
        })
    }
}

/// For the definition module, we want to expose the submodules as public items
/// at the root of the module for the documentation, and then add a get_definition function
/// that will return the constructed FDP definition
impl ToTokens for AppDefinitionModule {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (b_evn, b_evn_g) = (&self.broadcasted_events, &self.broadcasted_events.gen);
        let (i_req, i_req_g) = (&self.incoming_requests, &self.incoming_requests.gen);
        let (o_res, o_res_g) = (&self.outgoing_responses, &self.outgoing_responses.gen);
        let (l_evn, l_evn_g) = (&self.listened_events, &self.listened_events.gen);
        let (e_req, e_req_g) = (&self.emitted_requests, &self.emitted_requests.gen);

        tokens.extend(quote! {
            #[doc(inline)]
            pub use definition::*;

            // We keep the original submodules that keep the Rust functionality
            #[doc(hidden)]
            pub mod definition {
                #b_evn
                #i_req
                #o_res
                #l_evn
                #e_req
            }

            // We add a get_definition function that will return the constructed FDP definition
            // from the associated gen: TokenStream constructed to each submodule struct
            #[doc(hidden)]
            pub fn get_definition() -> fdp_common::info::AppDefinitionInfo {
                use fdp_common::info::*;
                use fdp_common::parsing::modules::*;

                // We expose all the messages
                use broadcasted_events::*;
                use incoming_requests::*;
                use outgoing_responses::*;
                use listened_events::*;
                use emitted_requests::*;

                AppDefinitionInfo {
                    broadcasted_events: #b_evn_g,
                    incoming_requests: #i_req_g,
                    outgoing_responses: #o_res_g,
                    listened_events: #l_evn_g,
                    emitted_requests: #e_req_g,
                }
            }
        });
    }
}

/// Enforces that a module had a specific name
fn enforce_module_name(module: &ItemMod, name: &str) -> syn::Result<()> {
    if module.ident != name {
        return Err(syn::Error::new_spanned(
            &module.ident,
            "The module must be named 'definition'",
        ));
    }
    Ok(())
}

struct Submodules {
    // Message declarations
    broadcasted_events: ItemMod,
    incoming_requests: ItemMod,
    outgoing_responses: ItemMod,
    // Message references
    listened_events: ItemMod,
    emitted_requests: ItemMod,
}

/// Enforces that a module contains only the required public submodules
fn enforce_submodules(module: &ItemMod) -> syn::Result<Submodules> {
    let required_submodules: Vec<String> = vec![
        "broadcasted_events",
        "incoming_requests",
        "outgoing_responses",
        "listened_events",
        "emitted_requests",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    // Error message closure for missing submodules
    let missing_modules = |submodules: &Vec<String>| {
        format!(
            "The fdp::definition module is missing the following public submodules: {}",
            submodules.join(", ")
        )
    };

    // Error message closure for unexpected items
    let unexpected_item = |ident: &str| {
        format!(
            "The fdp::definition module contains an unexpected item {}. It should only contain public submodules named: {:?}.",
            ident,required_submodules
        )
    };

    // Will hold the extracted submodules, which haven't been checked individually yet
    struct OptionalSubmodules {
        broadcasted_events: Option<ItemMod>,
        incoming_requests: Option<ItemMod>,
        outgoing_responses: Option<ItemMod>,
        listened_events: Option<ItemMod>,
        emitted_requests: Option<ItemMod>,
    }

    let mut submodules: OptionalSubmodules = OptionalSubmodules {
        broadcasted_events: None,
        incoming_requests: None,
        outgoing_responses: None,
        listened_events: None,
        emitted_requests: None,
    };

    let items = match &module.content {
        Some((_, items)) => items,
        None => {
            return Err(syn::Error::new_spanned(
                &module,
                missing_modules(&required_submodules),
            ))
        }
    };

    let mut found_submodules: Vec<String> = Vec::new();
    for item in items {
        if let syn::Item::Mod(item_mod) = item {
            // Ensure the module is public
            if !matches!(item_mod.vis, syn::Visibility::Public(_)) {
                return Err(syn::Error::new_spanned(
                    item_mod,
                    "The submodule must be public",
                ));
            }

            // Ensure the module has a valid name
            let submodule_name = item_mod.ident.to_string();
            if !required_submodules.contains(&submodule_name) {
                return Err(syn::Error::new_spanned(item_mod, unexpected_item(&quote! { #item }.to_string())));
            }

            found_submodules.push(submodule_name.clone());

            match submodule_name.as_str() {
                "broadcasted_events" => submodules.broadcasted_events = Some(item_mod.clone()),
                "incoming_requests" => submodules.incoming_requests = Some(item_mod.clone()),
                "outgoing_responses" => submodules.outgoing_responses = Some(item_mod.clone()),
                "listened_events" => submodules.listened_events = Some(item_mod.clone()),
                "emitted_requests" => submodules.emitted_requests = Some(item_mod.clone()),
                _ => {}
            }
        } else {
            return Err(syn::Error::new_spanned(
                item,
                unexpected_item(&quote! { #item }.to_string()),
            ));
        }
    }

    let missing_submodules: Vec<String> = required_submodules
        .iter()
        .filter(|&submodule| !found_submodules.contains(submodule))
        .cloned()
        .collect();
    if !missing_submodules.is_empty() {
        return Err(syn::Error::new_spanned(
            &module,
            format!(
                "The definition module is missing the following public submodules: {:?}",
                missing_submodules
            ),
        ));
    }

    // Unwrap all submodules and return them as a tuple
    Ok(Submodules {
        broadcasted_events: submodules.broadcasted_events.unwrap(),
        incoming_requests: submodules.incoming_requests.unwrap(),
        outgoing_responses: submodules.outgoing_responses.unwrap(),
        listened_events: submodules.listened_events.unwrap(),
        emitted_requests: submodules.emitted_requests.unwrap(),
    })
}



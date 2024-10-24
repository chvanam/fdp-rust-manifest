use syn::parse_quote;
use crate::parsing::definition::AppDefinitionModule;
use crate::parsing::modules::{
    BroadcastedEventsModule,
    ListenedEventsModule,
    IncomingRequestsModule,
    OutgoingResponsesModule,
    EmittedRequestsModule,
};

#[test]
fn test_parse_app_definition_module() {
    let input = parse_quote! {
        pub mod definition {
            pub mod broadcasted_events {
                pub struct Event1;
                pub struct Event2;
            }
            pub mod incoming_requests {
                pub struct Request1;
                pub struct Request2;
            }
            pub mod outgoing_responses {
                pub struct Response1;
                pub struct Response2;
            }
            pub mod listened_events {
                use crate::apps::other_app::broadcasted_events::OtherEvent;
            }
            pub mod emitted_requests {
                use crate::apps::other_app::incoming_requests::OtherRequest;
            }
        }
    };

    let result: syn::Result<AppDefinitionModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_broadcasted_events_module() {
    let input = parse_quote! {
        pub mod broadcasted_events {
            pub struct Event1;
            pub struct Event2 {
                field: String,
            }
        }
    };

    let result: syn::Result<BroadcastedEventsModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_listened_events_module() {
    let input = parse_quote! {
        pub mod listened_events {
            use crate::apps::other_app::broadcasted_events::OtherEvent1;
            use crate::apps::another_app::broadcasted_events::OtherEvent2;
        }
    };

    let result: syn::Result<ListenedEventsModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_incoming_requests_module() {
    let input = parse_quote! {
        pub mod incoming_requests {
            pub struct Request1;
            pub struct Request2 {
                field: i32,
            }
        }
    };

    let result: syn::Result<IncomingRequestsModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_outgoing_responses_module() {
    let input = parse_quote! {
        pub mod outgoing_responses {
            pub struct Response1;
            pub struct Response2 {
                field: bool,
            }
        }
    };

    let result: syn::Result<OutgoingResponsesModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_emitted_requests_module() {
    let input = parse_quote! {
        pub mod emitted_requests {
            use crate::apps::other_app::incoming_requests::OtherRequest1;
            use crate::apps::another_app::incoming_requests::OtherRequest2;
        }
    };

    let result: syn::Result<EmittedRequestsModule> = syn::parse2(input);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_app_definition_module() {
    let input = parse_quote! {
        pub mod definition {
            pub mod invalid_module {
                // This module shouldn't be here
            }
        }
    };

    let result: syn::Result<AppDefinitionModule> = syn::parse2(input);
    assert!(result.is_err());
}

#[test]
fn test_invalid_broadcasted_events_module() {
    let input = parse_quote! {
        pub mod broadcasted_events {
            fn invalid_item() {}
        }
    };

    let result: syn::Result<BroadcastedEventsModule> = syn::parse2(input);
    assert!(result.is_err());
}

#[test]
fn test_invalid_listened_events_module() {
    let input = parse_quote! {
        pub mod listened_events {
            use crate::invalid::path::Event;
        }
    };

    let result: syn::Result<ListenedEventsModule> = syn::parse2(input);
    assert!(result.is_err());
}
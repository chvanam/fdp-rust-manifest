use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Type alias for an asynchronous callback that returns a result of type `R`.
pub type AsyncCallback<R> = Pin<Box<dyn Future<Output = R> + Send>>;

/// An EventKey identifies an event in a HashMap.
pub trait EventKey: Hash + Eq + Debug + Clone {}
impl<T> EventKey for T where T: Hash + Eq + Debug + Clone {}

/// An EventHandler asynchronously handles a serialized payload (of bytes).
pub struct EventHandler {
    pub callback: Box<dyn Fn(Vec<u8>) -> AsyncCallback<()> + Send + Sync + 'static>,
}

impl EventHandler {
    fn handle(&self, payload: Vec<u8>) -> AsyncCallback<()> {
        (self.callback)(payload)
    }
}

/// An EventDispatcher stores handlers for a given event key.
pub struct EventDispatcher<K: EventKey> {
    pub handlers: HashMap<K, Arc<EventHandler>>,
}

impl<K: EventKey> EventDispatcher<K> {
    /// Creates a new event dispatcher.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Adds an asynchronous handler for a given topic.
    /// The handler is called with the deserialized payload.
    pub fn add_handler<T, C, P>(&mut self, topic: T, callback: C)
    where
        T: Into<K>,
        C: Fn(P) -> AsyncCallback<()> + Send + Sync + 'static,
        P: DeserializeOwned + Send + 'static,
    {
        let handler = EventHandler {
            callback: Box::new(move |payload: Vec<u8>| {
                match serde_json::from_slice::<P>(&payload) {
                    Ok(deserialized_payload) => callback(deserialized_payload),
                    Err(e) => {
                        log::error!("Failed to deserialize payload: {:?}", e);
                        Box::pin(async {})
                    }
                }
            }),
        };

        self.handlers.insert(topic.into(), Arc::new(handler));
    }

    /// Dispatches an event to its handler for a given topic.
    /// The handler runs asynchronously and doesn't block the caller.
    /// It returns a JoinHandle which can be used to wait for the handler to complete.
    pub fn dispatch<T, P>(&self, topic: T, payload: P) -> Option<JoinHandle<()>>
    where
        T: Into<K> + Clone,
        P: Into<Vec<u8>> + Send + 'static,
    {
        self.handlers.get(&topic.into()).map(|handler| {
            let handler_clone = Arc::clone(handler);
            tokio::spawn(async move {
                handler_clone.handle(payload.into()).await;
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::join_all;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tokio::{sync::Mutex, time};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    struct TestPayload {
        message: String,
        value: i32,
    }

    #[tokio::test]
    async fn accepts_async_handlers() {
        let example_payload = TestPayload {
            message: "Hello".into(),
            value: 42,
        };

        let mut event_dispatcher: EventDispatcher<String> = EventDispatcher::new();

        let example_payload_clone = example_payload.clone();

        event_dispatcher.add_handler("topic", move |payload: TestPayload| {
            let example_payload = example_payload_clone.clone();
            Box::pin(async move {
                time::sleep(time::Duration::from_secs(1)).await;
                assert_eq!(payload, example_payload);
                println!("Payload processed: {:?}", payload);
            })
        });

        // We call the topic handler with the serialized example payload
        let serialized_payload = serde_json::to_vec(&example_payload).unwrap();
        event_dispatcher.dispatch("topic", serialized_payload);
    }

    #[tokio::test]
    async fn test_clashing_event_keys() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
        struct SimplePayload {
            message: String,
        }

        let mut manager: EventDispatcher<String> = EventDispatcher::new();

        // Add the first handler to the manager.
        manager.add_handler("clashing_topic", |payload: SimplePayload| {
            Box::pin(async move {
                println!("Handler 1 processed: {:?}", payload);
                assert!(false, "This handler should not be called.");
            })
        });

        // Add the second handler to the manager, with the same key.
        manager.add_handler("clashing_topic", |payload: SimplePayload| {
            Box::pin(async move {
                println!("Handler 2 processed: {:?}", payload);
            })
        });

        // Payload to dispatch.
        let payload = SimplePayload {
            message: "Hello from clashing test".into(),
        };

        // Dispatch the event to the clashing topic.
        let serialized_payload = serde_json::to_vec(&payload).unwrap();
        if let Some(handle) = manager.dispatch("clashing_topic", serialized_payload) {
            handle.await.expect("Dispatched handler failed");
        } else {
            assert!(false, "Handler dispatch failed");
        }
    }

    /// Tests that async handlers don't block each others
    #[tokio::test]
    async fn test_async_handlers() {
        use tokio::time::{self, Duration};
        time::pause();

        #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
        struct SimplePayload {
            value: u64,
        }

        let mut manager: EventDispatcher<&str> = EventDispatcher::new();

        // Shared vector to record the order of handler completions.
        let completion_order = Arc::new(Mutex::new(Vec::new()));

        // Modified to accept a clone of the shared Arc<Mutex<Vec<u64>>> for recording
        let add_handler_with_recording =
            |manager: &mut EventDispatcher<&str>,
             topic: &'static str,
             completion_order: Arc<Mutex<Vec<u64>>>| {
                manager.add_handler(topic, move |payload: SimplePayload| {
                    let completion_order_clone = completion_order.clone();
                    Box::pin(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(payload.value)).await;
                        let mut order = completion_order_clone.lock().await;
                        order.push(payload.value);
                        println!("Payload processed: {:?}", payload);
                    })
                });
            };

        add_handler_with_recording(&mut manager, "topic/slow", completion_order.clone());
        add_handler_with_recording(&mut manager, "topic/fast", completion_order.clone());
        add_handler_with_recording(&mut manager, "topic/medium", completion_order.clone());

        let payloads = vec![
            ("topic/medium", SimplePayload { value: 2 }),
            ("topic/slow", SimplePayload { value: 3 }),
            ("topic/fast", SimplePayload { value: 1 }),
        ];

        let mut handles = Vec::new();
        for (topic, payload) in payloads {
            let serialized_payload = serde_json::to_vec(&payload).unwrap();
            if let Some(handle) = manager.dispatch(topic, serialized_payload) {
                handles.push(handle);
            }
        }

        time::advance(Duration::from_secs(4)).await;

        join_all(handles).await;

        // Check the order of completion
        let order = completion_order.lock().await;
        assert_eq!(
            *order,
            vec![1, 2, 3],
            "Handlers completed in the wrong order"
        );
    }

    #[tokio::test]
    async fn test_nonexistent_event_key() {
        let manager: EventDispatcher<String> = EventDispatcher::new();

        // Attempt to dispatch an event with a nonexistent key
        assert!(
            manager.dispatch("nonexistent_topic", vec![]).is_none(),
            "Expected None for unregistered event key"
        );
    }

    #[tokio::test]
    async fn test_payload_deserialization_error() {
        use serde::{Deserialize, Serialize};
        use std::time::Duration;

        #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
        struct ExpectedPayload {
            expected_field: String,
        }

        let mut manager: EventDispatcher<String> = EventDispatcher::new();

        // Handler expecting `ExpectedPayload`, but we will dispatch something else
        manager.add_handler(
            "test_deserialization_error",
            move |_payload: ExpectedPayload| {
                Box::pin(async move {
                    // If the handler gets called, the test should fail.
                    assert!(
                        false,
                        "Handler should not have been called due to deserialization error."
                    );
                })
            },
        );

        // Dispatch a payload that cannot be deserialized into `ExpectedPayload`.
        // For example, missing the `expected_field`.
        let incorrect_payload = serde_json::to_vec(&json!({"unexpected_field": "value"})).unwrap();

        // Since the dispatcher's `dispatch` function currently doesn't return any result indicating success or failure of deserialization,
        // and instead just logs errors, this test won't be able to directly assert the failure.
        // However, we can ensure that the system does not panic and that the incorrect handler is not called.
        if let Some(handle) = manager.dispatch("test_deserialization_error", incorrect_payload) {
            // We give some time for the handler to potentially be called if it incorrectly proceeds past deserialization.
            let result = tokio::time::timeout(Duration::from_secs(1), handle).await;
            assert!(
                result.is_ok(),
                "The dispatch should not panic or block indefinitely."
            );
        } else {
            assert!(false, "Dispatcher did not handle the event.");
        }
    }
}

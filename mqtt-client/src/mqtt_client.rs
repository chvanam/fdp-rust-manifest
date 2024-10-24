pub use crate::event_dispatcher::AsyncCallback;
use crate::event_dispatcher::{EventDispatcher, EventHandler};
pub use rumqttc::v5::EventLoop;
use rumqttc::v5::{
    mqttbytes::{
        v5::{Packet, Publish},
        QoS,
    },
    AsyncClient, Event, MqttOptions,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MqttClient {
    pub client: AsyncClient,
    event_dispatcher: Arc<Mutex<EventDispatcher<String>>>,
}

impl MqttClient {
    /// Creates a new MQTT client
    pub fn new(client_id: &str, host: &str, port: u16) -> (Self, EventLoop) {
        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(10));
        let (client, event_loop) = AsyncClient::new(mqttoptions, 10);
        let event_dispatcher: EventDispatcher<String> = EventDispatcher::new();

        (
            MqttClient {
                client,
                event_dispatcher: Arc::new(Mutex::new(event_dispatcher)),
            },
            event_loop,
        )
    }

    /// Publishes a payload to a topic
    pub async fn publish<T, P>(&self, topic: T, payload: P)
    where
        T: Into<String>,
        P: Serialize,
    {
        let serialized_payload = serde_json::to_vec(&payload).unwrap();
        self.client
            .publish(topic, QoS::AtMostOnce, false, serialized_payload)
            .await
            .unwrap();
    }

    ///
    pub async fn register_callback<F, P>(&mut self, topic: &str, callback: F)
    where
        P: DeserializeOwned + Send + 'static,
        F: Fn(P) -> AsyncCallback<()> + Send + Sync + 'static,
    {
        self.client
            .subscribe(topic, QoS::AtLeastOnce)
            .await
            .unwrap();

        self.event_dispatcher
            .lock()
            .await
            .add_handler(topic, callback);
    }

    pub async fn register_response<C, P, R>(&mut self, topic: &str, reply_topic: &str, callback: C)
    where
        C: Fn(P) -> AsyncCallback<R> + Send + Sync + Clone + 'static,
        P: DeserializeOwned + Send + 'static,
        R: Serialize + Send + 'static,
    {
        self.client
            .subscribe(topic, QoS::AtLeastOnce)
            .await
            .unwrap();

        let client_clone = self.clone();
        let reply_topic = reply_topic.to_owned();
        let handler = EventHandler {
            callback: Box::new(move |payload: Vec<u8>| {
                match serde_json::from_slice::<P>(&payload) {
                    Ok(deserialized_payload) => {
                        let responder = client_clone.clone();
                        let callback_clone = callback.clone();
                        let reply_topic_clone = reply_topic.clone();
                        Box::pin(async move {
                            let response = callback_clone(deserialized_payload).await;
                            responder.publish(reply_topic_clone, response).await;
                        })
                    }
                    Err(e) => {
                        log::error!("Failed to deserialize payload: {:?}", e);
                        Box::pin(async {})
                    }
                }
            }),
        };

        self.event_dispatcher
            .lock()
            .await
            .handlers
            .insert(topic.into(), Arc::new(handler));
    }

    pub async fn start(self, mut event_loop: EventLoop) {
        // tokio::spawn(async move {
        loop {
            let event = event_loop.poll().await.unwrap();
            if let Event::Incoming(Packet::Publish(Publish { topic, payload, .. })) = event {
                let topic_str = String::from_utf8(topic.to_vec()).unwrap();
                self.event_dispatcher
                    .lock()
                    .await
                    .dispatch(topic_str, payload);
            }
        }
        // });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fmt::Debug, time::Duration};
    use tokio::{process::Command, sync::mpsc, task::JoinHandle, time::sleep};

    /// Helper function to publish a message to a given topic using mosquitto_pub, with serialization.
    async fn mock_publish<T, P>(topic: T, payload: P)
    where
        T: Into<String>,
        P: Serialize + Debug,
    {
        let topic = topic.into();
        let serialized_message =
            serde_json::to_string(&payload).expect("Failed to serialize message");
        println!(
            "🥸  Topic: {} | Mock publishing message: {}",
            topic, serialized_message
        );
        Command::new("mosquitto_pub")
            .arg("-t")
            .arg(topic)
            .arg("-m")
            .arg(serialized_message)
            .spawn()
            .expect("mosquitto_pub command failed to start")
            .wait()
            .await
            .expect("Failed to wait on mosquitto_pub");
    }

    /// Helper function to assert that a message is received on a given topic, with deserialization.
    async fn assert_received<T, P>(topic: T, expected_payload: P) -> JoinHandle<()>
    where
        T: Into<String> + Send + Copy + 'static,
        P: Serialize + Send + Debug + 'static,
    {
        let handler = tokio::spawn(async move {
            println!(
                "🔎 Topic: {} | Waiting for message {:?}",
                topic.into(),
                expected_payload
            );

            let child = Command::new("mosquitto_sub")
                .arg("-C")
                .arg("1")
                .arg("-t")
                .arg(topic.into())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to spawn mosquitto_sub");

            let output = child
                .wait_with_output()
                .await
                .expect("Failed to wait on mosquitto_sub");

            let expected_payload_serialized = serde_json::to_string(&expected_payload).unwrap();
            let received_payload_serialized = String::from_utf8(output.stdout).unwrap();
            let received_payload_serialized = received_payload_serialized.trim();

            println!(
                "📩 Topic: {} | Received message: {:?}",
                topic.into(),
                received_payload_serialized
            );

            assert_eq!(
                received_payload_serialized, expected_payload_serialized,
                "The expected message was not received for the given topic."
            );
        });

        sleep(Duration::from_millis(100)).await;

        handler
    }

    #[tokio::test]
    async fn test_mqtt_testing_flow() {
        let topic = "test/flow";
        let payload = "Hello flow";

        let receiver = assert_received(topic, payload).await;
        mock_publish(topic, payload).await;
        receiver.await.expect("The spawned task failed");
    }

    #[tokio::test]
    async fn callback() {
        let (mut client, event_loop) = MqttClient::new("callback", "localhost", 1883);
        let topic = "test/topic";
        let payload = String::from("Hello World");
        let serialized_payload = serde_json::to_string(&payload).unwrap();

        let (tx, mut rx) = mpsc::channel::<String>(1);

        client
            .register_callback(topic, move |p: String| {
                println!("Callback with {}", p);
                let tx_clone = tx.clone();
                Box::pin(async move {
                    let _ = tx_clone.send(p).await;
                })
            })
            .await;

        tokio::spawn(async move {
            client.start(event_loop).await;
        });

        mock_publish(topic, &serialized_payload).await;
        let received_message = rx.recv().await.expect("Failed to receive message");
        assert_eq!(received_message, serialized_payload);
    }

    #[tokio::test]
    async fn response() {
        let (mut client, event_loop) = MqttClient::new("response", "localhost", 1883);
        let topic = "test/topic";
        let reply_topic = "test/reply";
        let payload = 42;
        let reply_payload = "success";

        let (tx, mut rx) = mpsc::channel::<i32>(1);

        client
            .register_response::<_, _, String>(topic, reply_topic, move |p: i32| {
                let tx_clone = tx.clone();
                Box::pin(async move {
                    let _ = tx_clone.send(p).await;
                    reply_payload.to_string()
                })
            })
            .await;

        tokio::spawn(async move {
            client.start(event_loop).await;
        });

        let receiver = assert_received(reply_topic, reply_payload.to_string()).await;
        mock_publish(topic, &payload).await;
        let received_message = rx.recv().await.expect("Failed to receive message");
        assert_eq!(received_message, payload);
        receiver.await.expect("The spawned task failed");
    }
}

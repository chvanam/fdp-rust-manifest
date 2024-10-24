//! A mqtt-client wrapper designed to be used with the FDP system

use fdp_common::mqtt::{Event, Message, Request};
use mqtt_client::{AsyncCallback, EventLoop, MqttClient as RawMqttClient};

#[derive(Clone)]
pub struct MqttClient {
    pub client: RawMqttClient,
}

impl MqttClient {
    /// Creates a new instance of the MqttClient.
    pub fn new(client_id: &str, host: &str, port: u16) -> (Self, EventLoop) {
        let (client, event_loop) = RawMqttClient::new(client_id, host, port);
        (MqttClient { client }, event_loop)
    }

    /// Broadcasts an Event
    pub async fn broadcast<E: Event>(&self, event: E) {
        self.client.publish(E::topic(), event).await;
    }

    /// Emit a Request
    pub async fn request<R: Request>(&self, request: R) {
        self.client.publish(R::topic(), request).await;
    }

    /// Registers an Event listener
    pub async fn listen<C, E>(&mut self, callback: C)
    where
        C: Fn(E) -> AsyncCallback<()> + Send + Sync + 'static,
        E: Event,
    {
        self.client.register_callback(E::topic(), callback).await;
    }

    /// Register a Request handler
    /// The response type can be obtained using the following syntax:
    /// ```compile_fail
    /// type Response = <MyIncomingRequest as Request>::Response;
    /// ```
    pub async fn respond<C, R>(&mut self, callback: C)
    where
        C: Fn(R) -> AsyncCallback<R::Response> + Send + Sync + Clone + 'static,
        R: Request,
    {
        self.client
            .register_response::<_, _, R::Response>(R::topic(), R::Response::topic(), callback)
            .await;
    }

    /// Starts the MQTT client and begins processing incoming messages.
    pub async fn start(self, event_loop: EventLoop) {
        self.client.start(event_loop).await;
    }
}

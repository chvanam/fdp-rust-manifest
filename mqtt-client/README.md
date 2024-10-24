# MQTT Client

A Rust library providing a simple asynchronous MQTT client wrapper arround `rumqttc`.

## Features

- Publish messages to MQTT topics
- Register callbacks for topic/payloads
- Automatic serialization and deserialization of payloads

## Tests

The tests require the commands `mosquitto_pub` and `mosquitto_sub` to be installed.

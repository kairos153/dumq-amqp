# DUMQ-AMQP - AMQP 1.0 Protocol Implementation in Rust

[![Crates.io](https://img.shields.io/crates/v/dumq-amqp)](https://crates.io/crates/dumq-amqp)
[![Documentation](https://docs.rs/dumq-amqp/badge.svg)](https://docs.rs/dumq-amqp)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A complete implementation of the AMQP 1.0 messaging protocol in Rust, providing both client and server capabilities with full async/await support.

## Features

- **Full AMQP 1.0 Protocol Support**: Complete implementation of the AMQP 1.0 specification
- **Async/Await**: Built on top of Tokio for high-performance async operations
- **Type Safety**: Strongly typed AMQP values and messages
- **Builder Pattern**: Fluent builder APIs for easy configuration
- **Error Handling**: Comprehensive error types with detailed error messages
- **Extensible**: Modular design for easy extension and customization
- **Message Encoding/Decoding**: Efficient binary encoding and decoding of AMQP values
- **Connection Management**: Robust connection lifecycle management
- **Session and Link Management**: Complete session and link handling with flow control

## Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
dumq-amqp = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection
    let connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("my-app")
        .build();

    println!("Connection created with ID: {}", connection.id());
    println!("Connection state: {:?}", connection.state());

    // Create a message
    let message = Message::text("Hello, AMQP!");
    println!("Message created: {:?}", message.body_as_text());

    Ok(())
}
```

## Core Concepts

### AMQP Values

The library provides strongly typed AMQP values:

```rust
use dumq_amqp::types::AmqpValue;

let values = vec![
    AmqpValue::String("Hello".to_string()),
    AmqpValue::Int(42),
    AmqpValue::Boolean(true),
    AmqpValue::Double(3.14159),
    AmqpValue::Uuid(uuid::Uuid::new_v4()),
    AmqpValue::Binary(vec![1, 2, 3, 4]),
];
```

### Messages

Create and manipulate AMQP messages:

```rust
use dumq_amqp::message::Message;

// Simple text message
let text_msg = Message::text("Hello, World!");

// Binary message
let binary_msg = Message::binary(b"Binary data");

// Message with properties
let complex_msg = Message::builder()
    .build()
    .with_message_id("msg-001")
    .with_subject("Test Message");
```

### Encoding/Decoding

The library includes efficient binary encoding and decoding:

```rust
use dumq_amqp::codec::{Encoder, Decoder};

let value = AmqpValue::String("Hello, AMQP!".to_string());

// Encode
let mut encoder = Encoder::new();
encoder.encode_value(&value)?;
let encoded = encoder.finish();

// Decode
let mut decoder = Decoder::new(encoded);
let decoded = decoder.decode_value()?;
assert_eq!(value, decoded);
```

## Architecture

The library is organized into several key modules:

- **`connection`**: Connection management and lifecycle
- **`session`**: Session handling and flow control
- **`link`**: Sender and receiver link management
- **`message`**: AMQP message structures and manipulation
- **`types`**: AMQP value types and data structures
- **`codec`**: Binary encoding and decoding
- **`transport`**: Low-level transport layer
- **`error`**: Comprehensive error handling
- **`condition`**: AMQP error conditions and handling

## Examples

The library includes several comprehensive examples demonstrating different use cases:

### Basic Example

```bash
cargo run --example basic
```

Demonstrates basic connection creation and message handling.

### Producer Example

```bash
cargo run --example producer
```

Shows how to create a producer that sends multiple messages with different properties.

### Consumer Example

```bash
cargo run --example consumer
```

Demonstrates message consumption with different message types and processing logic.

### Simple Broker Example

```bash
cargo run --example simple_broker
```

Shows a basic AMQP broker implementation with queue management and message routing.

### Publisher Example

```bash
cargo run --example publisher
```

Demonstrates publishing messages with various content types and properties.

### Encoding/Decoding Example

```bash
cargo run --example encoding_decoding
```

Shows how to encode and decode AMQP values and messages.

### Error Handling Example

```bash
cargo run --example error_handling
```

Demonstrates comprehensive error handling patterns.

### Network Connection Example

```bash
cargo run --example network_connection
```

Shows how to establish actual TCP connections and perform AMQP protocol negotiation.

### Network Integration Examples

The following examples now support actual network connections:

```bash
# Start the broker server
cargo run --example simple_broker 5673

# In another terminal, run publisher
cargo run --example publisher localhost 5673

# In another terminal, run consumer
cargo run --example consumer localhost 5673
```

These examples demonstrate real network connectivity with fallback to simulation mode when no broker is available.

## API Reference

### Core Types

#### AmqpValue
The main enum representing all AMQP 1.0 value types:

```rust
use dumq_amqp::types::AmqpValue;

let values = vec![
    AmqpValue::String("Hello".to_string()),
    AmqpValue::Int(42),
    AmqpValue::Boolean(true),
    AmqpValue::Double(3.14159),
    AmqpValue::Uuid(uuid::Uuid::new_v4()),
    AmqpValue::Binary(vec![1, 2, 3, 4]),
];
```

#### Message
AMQP 1.0 message structure with support for various content types:

```rust
use dumq_amqp::message::Message;

// Simple text message
let text_msg = Message::text("Hello, World!");

// Binary message
let binary_msg = Message::binary(b"Binary data");

// Complex message with properties
let complex_msg = Message::builder()
    .build()
    .with_message_id("msg-001")
    .with_subject("Test Message");
```

### Connection Management

#### ConnectionBuilder
Fluent builder for creating AMQP connections:

```rust
use dumq_amqp::connection::ConnectionBuilder;
use tokio::time::Duration;

let connection = ConnectionBuilder::new()
    .hostname("localhost")
    .port(5672)
    .timeout(Duration::from_secs(30))
    .max_frame_size(65536)
    .channel_max(1000)
    .idle_timeout(Duration::from_secs(60))
    .container_id("my-application")
    .build();
```

#### Connection Lifecycle
```rust
use dumq_amqp::connection::Connection;

let connection = ConnectionBuilder::new()
    .hostname("localhost")
    .port(5672)
    .build();

println!("Connection ID: {}", connection.id());
println!("Connection state: {:?}", connection.state());
```

### Session Management

#### Session
AMQP sessions provide flow control and link management:

```rust
use dumq_amqp::session::Session;

// Create session
let session = Session::new(1, "test-connection".to_string());
println!("Session ID: {}", session.id());
println!("Session state: {:?}", session.state());
```

### Link Management

#### LinkBuilder
Create sender and receiver links:

```rust
use dumq_amqp::link::{LinkBuilder, Sender, Receiver};

// Create sender
let sender = LinkBuilder::new()
    .name("my-sender")
    .target("my-queue")
    .build_sender("session-id".to_string());

// Create receiver
let receiver = LinkBuilder::new()
    .name("my-receiver")
    .source("my-queue")
    .build_receiver("session-id".to_string());
```

### Encoding/Decoding

#### Codec
Efficient binary encoding and decoding:

```rust
use dumq_amqp::codec::{Encoder, Decoder};
use dumq_amqp::types::AmqpValue;

// Encode
let value = AmqpValue::String("Hello, AMQP!".to_string());
let mut encoder = Encoder::new();
encoder.encode_value(&value)?;
let encoded = encoder.finish();

// Decode
let mut decoder = Decoder::new(encoded);
let decoded = decoder.decode_value()?;
assert_eq!(value, decoded);
```

### Error Handling

#### AmqpError
Comprehensive error types for AMQP operations:

```rust
use dumq_amqp::error::{AmqpError, AmqpResult};

fn handle_amqp_operation() -> AmqpResult<()> {
    match some_operation() {
        Ok(result) => Ok(result),
        Err(AmqpError::Connection(msg)) => {
            eprintln!("Connection error: {}", msg);
            Err(AmqpError::connection("Failed to connect"))
        }
        Err(AmqpError::Timeout(msg)) => {
            eprintln!("Timeout error: {}", msg);
            Err(AmqpError::timeout("Operation timed out"))
        }
        Err(e) => Err(e),
    }
}
```

### Network Management

#### NetworkConnection
Establish and manage AMQP network connections:

```rust
use dumq_amqp::network::{NetworkConnection, NetworkBuilder};
use tokio::time::Duration;

let mut connection = NetworkBuilder::new()
    .hostname("localhost")
    .port(5672)
    .timeout(Duration::from_secs(30))
    .keep_alive(Duration::from_secs(60))
    .container_id("my-app")
    .build();

// Connect to remote host
connection.connect().await?;

// Negotiate AMQP protocol
connection.negotiate_protocol().await?;

// Send and receive messages
let message = Message::text("Hello, AMQP!");
connection.send_message(0, &message).await?;

let received = connection.receive_message().await?;

// Disconnect
connection.disconnect().await?;
```

#### NetworkBuilder
Fluent builder for network connections:

```rust
use dumq_amqp::network::NetworkBuilder;
use tokio::time::Duration;

let connection = NetworkBuilder::new()
    .hostname("my-broker.example.com")
    .port(5672)
    .timeout(Duration::from_secs(30))
    .keep_alive(Duration::from_secs(60))
    .max_frame_size(65536)
    .channel_max(1000)
    .idle_timeout(Duration::from_secs(60))
    .container_id("my-application")
    .property("product".to_string(), AmqpValue::String("MyApp".to_string()))
    .build();
```

## Testing

Run the examples:

```bash
cargo run --example basic
cargo run --example producer
cargo run --example consumer
cargo run --example simple_broker
cargo run --example publisher
cargo run --example encoding_decoding
cargo run --example error_handling
cargo run --example network_connection
```

Run tests:

```bash
cargo test
```

## Documentation

Generate documentation:

```bash
cargo doc --no-deps --open
```

## Current Status

This library is currently in development (version 0.1.0) and includes:

### ✅ Implemented Features
- Complete AMQP 1.0 value type system
- Message creation and manipulation
- Binary encoding and decoding
- Connection and session management
- Link creation and configuration
- Comprehensive error handling
- Builder pattern APIs
- Async/await support with Tokio
- **Network layer with TCP connections**
- **AMQP protocol negotiation**
- **Frame encoding and decoding**
- **Message transmission and reception**
- **Connection keep-alive and heartbeat**
- **Network integration examples with real connectivity**
- Multiple example implementations

### 🔄 In Progress
- Complete connection lifecycle management
- Flow control implementation
- Performance optimizations

### 📋 Planned Features
- SASL authentication support
- TLS/SSL support
- Connection pooling
- Message persistence
- Transaction support
- More comprehensive examples
- Benchmarking suite

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- AMQP 1.0 specification
- Tokio async runtime
- Rust community# Updated at Tue Aug  5 19:54:52 KST 2025

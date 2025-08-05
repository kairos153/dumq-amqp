# dumq_amqp API Documentation

## Overview

dumq_amqp is a complete implementation of the AMQP 1.0 messaging protocol in Rust. This document provides detailed API reference for all modules and types.

## Table of Contents

- [Core Types](#core-types)
- [Connection Management](#connection-management)
- [Session Management](#session-management)
- [Link Management](#link-management)
- [Message System](#message-system)
- [Encoding/Decoding](#encodingdecoding)
- [Error Handling](#error-handling)
- [Transport Layer](#transport-layer)

## Core Types

### AmqpValue

The main enum representing all AMQP 1.0 value types.

```rust
pub enum AmqpValue {
    Null,
    Boolean(bool),
    Ubyte(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    Timestamp(i64),
    Uuid(uuid::Uuid),
    Binary(Vec<u8>),
    String(String),
    Symbol(AmqpSymbol),
    List(AmqpList),
    Map(AmqpMap),
    Array(Vec<AmqpValue>),
}
```

#### Examples

```rust
use dumq_amqp::types::AmqpValue;

// Primitive types
let null = AmqpValue::Null;
let boolean = AmqpValue::Boolean(true);
let integer = AmqpValue::Int(42);
let float = AmqpValue::Double(3.14159);
let string = AmqpValue::String("Hello".to_string());

// Complex types
let uuid = AmqpValue::Uuid(uuid::Uuid::new_v4());
let binary = AmqpValue::Binary(vec![1, 2, 3, 4]);
```

### AmqpSymbol

Optimized string type for frequently used identifiers.

```rust
pub struct AmqpSymbol(pub String);

impl AmqpSymbol {
    pub fn as_str(&self) -> &str;
}
```

#### Examples

```rust
use dumq_amqp::types::AmqpSymbol;

let symbol = AmqpSymbol::from("my-symbol");
assert_eq!(symbol.as_str(), "my-symbol");
```

### AmqpList and AmqpMap

Composite types for structured data.

```rust
pub struct AmqpList(pub Vec<AmqpValue>);
pub struct AmqpMap(pub HashMap<AmqpSymbol, AmqpValue>);
```

#### Examples

```rust
use dumq_amqp::types::{AmqpList, AmqpMap, AmqpValue, AmqpSymbol};
use std::collections::HashMap;

// Create a list
let list = AmqpList::from(vec![
    AmqpValue::String("item1".to_string()),
    AmqpValue::Int(42),
    AmqpValue::Boolean(true),
]);

// Create a map
let mut map_data = HashMap::new();
map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(123));
let map = AmqpMap::from(map_data);
```

## Connection Management

### Connection

Represents an AMQP 1.0 connection to a broker.

```rust
pub struct Connection {
    state: ConnectionState,
    config: ConnectionConfig,
    stream: Option<TcpStream>,
    id: String,
    next_channel: u16,
    sessions: HashMap<u16, Session>,
}

impl Connection {
    pub fn new(config: ConnectionConfig) -> Self;
    pub async fn open(&mut self) -> AmqpResult<()>;
    pub async fn close(&mut self) -> AmqpResult<()>;
    pub async fn create_session(&mut self) -> AmqpResult<Session>;
    pub fn state(&self) -> &ConnectionState;
    pub fn id(&self) -> &str;
}
```

### ConnectionState

Represents the current state of a connection.

```rust
pub enum ConnectionState {
    Opening,
    Open,
    Closing,
    Closed,
    Error(String),
}
```

### ConnectionConfig

Configuration for AMQP connections.

```rust
pub struct ConnectionConfig {
    pub hostname: String,
    pub port: u16,
    pub timeout: Duration,
    pub max_frame_size: u32,
    pub channel_max: u16,
    pub idle_timeout: Duration,
    pub container_id: String,
    pub properties: HashMap<String, AmqpValue>,
}
```

### ConnectionBuilder

Fluent builder for creating connections.

```rust
pub struct ConnectionBuilder {
    config: ConnectionConfig,
}

impl ConnectionBuilder {
    pub fn new() -> Self;
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self;
    pub fn port(mut self, port: u16) -> Self;
    pub fn timeout(mut self, timeout: Duration) -> Self;
    pub fn max_frame_size(mut self, max_frame_size: u32) -> Self;
    pub fn channel_max(mut self, channel_max: u16) -> Self;
    pub fn idle_timeout(mut self, idle_timeout: Duration) -> Self;
    pub fn container_id(mut self, container_id: impl Into<String>) -> Self;
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self;
    pub fn build(self) -> Connection;
}
```

#### Examples

```rust
use dumq_amqp::connection::{Connection, ConnectionBuilder};
use tokio::time::Duration;

// Create a connection
let mut connection = ConnectionBuilder::new()
    .hostname("localhost")
    .port(5672)
    .timeout(Duration::from_secs(30))
    .container_id("my-app")
    .build();

// Open the connection
connection.open().await?;

// Use the connection...

// Close the connection
connection.close().await?;
```

## Session Management

### Session

Represents an AMQP 1.0 session within a connection.

```rust
pub struct Session {
    config: SessionConfig,
    state: SessionState,
    id: String,
    connection_id: String,
    channel: u16,
    links: HashMap<String, Link>,
    next_handle: u32,
}

impl Session {
    pub fn new(channel: u16, connection_id: String) -> Self;
    pub async fn begin(&mut self) -> AmqpResult<()>;
    pub async fn end(&mut self) -> AmqpResult<()>;
    pub async fn create_sender(&mut self, config: LinkConfig) -> AmqpResult<Sender>;
    pub async fn create_receiver(&mut self, config: LinkConfig) -> AmqpResult<Receiver>;
    pub fn state(&self) -> &SessionState;
    pub fn id(&self) -> &str;
    pub fn channel(&self) -> u16;
}
```

### SessionState

Represents the current state of a session.

```rust
pub enum SessionState {
    Beginning,
    Active,
    Ending,
    Ended,
    Error(String),
}
```

#### Examples

```rust
use dumq_amqp::session::Session;

// Create session from connection
let mut session = connection.create_session().await?;

// Begin session
session.begin().await?;

// Use session...

// End session
session.end().await?;
```

## Link Management

### Link

Base trait for AMQP links (senders and receivers).

```rust
pub trait Link {
    fn name(&self) -> &str;
    fn state(&self) -> &LinkState;
    async fn attach(&mut self) -> AmqpResult<()>;
    async fn detach(&mut self) -> AmqpResult<()>;
}
```

### Sender

Represents an AMQP sender link for sending messages.

```rust
pub struct Sender {
    config: LinkConfig,
    state: LinkState,
    credit: u32,
    // ... other fields
}

impl Sender {
    pub fn new(config: LinkConfig) -> Self;
    pub async fn attach(&mut self) -> AmqpResult<()>;
    pub async fn detach(&mut self) -> AmqpResult<()>;
    pub async fn send(&mut self, message: Message) -> AmqpResult<u32>;
    pub fn add_credit(&mut self, credit: u32);
    pub fn credit(&self) -> u32;
}
```

### Receiver

Represents an AMQP receiver link for receiving messages.

```rust
pub struct Receiver {
    config: LinkConfig,
    state: LinkState,
    credit: u32,
    // ... other fields
}

impl Receiver {
    pub fn new(config: LinkConfig) -> Self;
    pub async fn attach(&mut self) -> AmqpResult<()>;
    pub async fn detach(&mut self) -> AmqpResult<()>;
    pub async fn receive(&mut self) -> AmqpResult<Option<Message>>;
    pub fn add_credit(&mut self, credit: u32);
    pub fn credit(&self) -> u32;
}
```

### LinkConfig

Configuration for AMQP links.

```rust
pub struct LinkConfig {
    pub name: String,
    pub source: Option<String>,
    pub target: Option<String>,
    pub sender_settle_mode: SenderSettleMode,
    pub receiver_settle_mode: ReceiverSettleMode,
    pub properties: HashMap<String, AmqpValue>,
}
```

### LinkBuilder

Fluent builder for creating links.

```rust
pub struct LinkBuilder {
    config: LinkConfig,
}

impl LinkBuilder {
    pub fn new() -> Self;
    pub fn name(mut self, name: impl Into<String>) -> Self;
    pub fn source(mut self, source: impl Into<String>) -> Self;
    pub fn target(mut self, target: impl Into<String>) -> Self;
    pub fn sender_settle_mode(mut self, mode: SenderSettleMode) -> Self;
    pub fn receiver_settle_mode(mut self, mode: ReceiverSettleMode) -> Self;
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self;
    pub fn build_sender(self, session_id: String) -> Sender;
    pub fn build_receiver(self, session_id: String) -> Receiver;
}
```

#### Examples

```rust
use dumq_amqp::link::{LinkBuilder, Sender, Receiver};

// Create a sender
let mut sender = LinkBuilder::new()
    .name("my-sender")
    .target("my-queue")
    .build_sender(session.id().to_string());

sender.attach().await?;
sender.add_credit(10);

let message = Message::text("Hello, AMQP!");
let delivery_id = sender.send(message).await?;

sender.detach().await?;

// Create a receiver
let mut receiver = LinkBuilder::new()
    .name("my-receiver")
    .source("my-queue")
    .build_receiver(session.id().to_string());

receiver.attach().await?;
receiver.add_credit(10);

while let Ok(Some(message)) = receiver.receive().await {
    if let Some(text) = message.body_as_text() {
        println!("Received: {}", text);
    }
}

receiver.detach().await?;
```

## Message System

### Message

Represents an AMQP 1.0 message.

```rust
pub struct Message {
    pub header: Option<Header>,
    pub delivery_annotations: Option<AmqpMap>,
    pub message_annotations: Option<AmqpMap>,
    pub properties: Option<Properties>,
    pub application_properties: Option<AmqpMap>,
    pub body: Option<Body>,
    pub footer: Option<AmqpMap>,
}

impl Message {
    pub fn builder() -> MessageBuilder;
    pub fn text(text: impl Into<String>) -> Self;
    pub fn binary(data: impl Into<Vec<u8>>) -> Self;
    pub fn body_as_text(&self) -> Option<&str>;
    pub fn body_as_binary(&self) -> Option<&[u8]>;
    pub fn message_id_as_string(&self) -> Option<String>;
    pub fn with_message_id(mut self, id: impl Into<String>) -> Self;
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self;
    pub fn with_content_type(mut self, content_type: impl Into<AmqpSymbol>) -> Self;
}
```

### Header

Message header containing delivery-related information.

```rust
pub struct Header {
    pub durable: Option<bool>,
    pub priority: Option<u8>,
    pub ttl: Option<u32>,
    pub first_acquirer: Option<bool>,
    pub delivery_count: Option<u32>,
}
```

### Properties

Message properties containing application-level metadata.

```rust
pub struct Properties {
    pub message_id: Option<AmqpValue>,
    pub user_id: Option<Vec<u8>>,
    pub to: Option<String>,
    pub subject: Option<String>,
    pub reply_to: Option<String>,
    pub correlation_id: Option<AmqpValue>,
    pub content_type: Option<AmqpSymbol>,
    pub content_encoding: Option<AmqpSymbol>,
    pub absolute_expiry_time: Option<i64>,
    pub creation_time: Option<i64>,
    pub group_id: Option<String>,
    pub group_sequence: Option<u32>,
    pub reply_to_group_id: Option<String>,
}
```

### Body

Message body content.

```rust
pub enum Body {
    Data(Vec<u8>),
    Value(AmqpValue),
    Sequence(AmqpList),
    Multiple(Vec<Body>),
}
```

### MessageBuilder

Fluent builder for creating messages.

```rust
pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    pub fn new() -> Self;
    pub fn header(mut self, header: Header) -> Self;
    pub fn delivery_annotations(mut self, annotations: AmqpMap) -> Self;
    pub fn message_annotations(mut self, annotations: AmqpMap) -> Self;
    pub fn properties(mut self, properties: Properties) -> Self;
    pub fn application_properties(mut self, properties: AmqpMap) -> Self;
    pub fn body(mut self, body: Body) -> Self;
    pub fn footer(mut self, footer: AmqpMap) -> Self;
    pub fn build(self) -> Message;
}
```

#### Examples

```rust
use dumq_amqp::message::{Message, MessageBuilder};
use dumq_amqp::types::{AmqpValue, AmqpSymbol};

// Simple text message
let text_msg = Message::text("Hello, World!");

// Binary message
let binary_msg = Message::binary(b"Binary data");

// Complex message with properties
let complex_msg = Message::builder()
    .body(message::Body::Value(AmqpValue::String("Custom content".to_string())))
    .build()
    .with_message_id("msg-001")
    .with_subject("Test Message")
    .with_content_type(AmqpSymbol::from("text/plain"));

// Access message content
if let Some(text) = text_msg.body_as_text() {
    println!("Message text: {}", text);
}

if let Some(binary) = binary_msg.body_as_binary() {
    println!("Message binary: {:?}", binary);
}
```

## Encoding/Decoding

### Encoder

Encodes AMQP values to binary format.

```rust
pub struct Encoder {
    buffer: BytesMut,
}

impl Encoder {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;
    pub fn encode_value(&mut self, value: &AmqpValue) -> Result<(), AmqpError>;
    pub fn encode_null(&mut self) -> Result<(), AmqpError>;
    pub fn encode_boolean(&mut self, value: bool) -> Result<(), AmqpError>;
    pub fn encode_int(&mut self, value: i32) -> Result<(), AmqpError>;
    pub fn encode_string(&mut self, value: &str) -> Result<(), AmqpError>;
    // ... other encode methods
    pub fn finish(self) -> Vec<u8>;
}
```

### Decoder

Decodes binary data to AMQP values.

```rust
pub struct Decoder {
    buffer: Bytes,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Self;
    pub fn decode_value(&mut self) -> Result<AmqpValue, AmqpError>;
    pub fn has_remaining(&self) -> bool;
    pub fn remaining(&self) -> usize;
}
```

#### Examples

```rust
use dumq_amqp::codec::{Encoder, Decoder};
use dumq_amqp::types::AmqpValue;

// Encode a value
let value = AmqpValue::String("Hello, AMQP!".to_string());
let mut encoder = Encoder::new();
encoder.encode_value(&value)?;
let encoded = encoder.finish();

// Decode the value
let mut decoder = Decoder::new(encoded);
let decoded = decoder.decode_value()?;
assert_eq!(value, decoded);

// Encode multiple values
let mut encoder = Encoder::new();
encoder.encode_value(&AmqpValue::String("Hello".to_string()))?;
encoder.encode_value(&AmqpValue::Int(42))?;
encoder.encode_value(&AmqpValue::Boolean(true))?;
let encoded = encoder.finish();
```

## Error Handling

### AmqpError

Comprehensive error types for AMQP operations.

```rust
pub enum AmqpError {
    Connection(String),
    Session(String),
    Link(String),
    Transport(String),
    Encoding(String),
    Decoding(String),
    Protocol(String),
    Timeout(String),
    Io(#[from] std::io::Error),
    Serialization(#[from] serde_json::Error),
    InvalidState(String),
    NotImplemented(String),
}

impl AmqpError {
    pub fn connection(msg: impl Into<String>) -> Self;
    pub fn session(msg: impl Into<String>) -> Self;
    pub fn link(msg: impl Into<String>) -> Self;
    pub fn transport(msg: impl Into<String>) -> Self;
    pub fn encoding(msg: impl Into<String>) -> Self;
    pub fn decoding(msg: impl Into<String>) -> Self;
    pub fn protocol(msg: impl Into<String>) -> Self;
    pub fn timeout(msg: impl Into<String>) -> Self;
    pub fn invalid_state(msg: impl Into<String>) -> Self;
    pub fn not_implemented(msg: impl Into<String>) -> Self;
}
```

### AmqpResult

Result type for AMQP operations.

```rust
pub type AmqpResult<T> = Result<T, AmqpError>;
```

#### Examples

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

// Create specific error types
let conn_error = AmqpError::connection("Failed to establish connection");
let timeout_error = AmqpError::timeout("Operation timed out");
let state_error = AmqpError::invalid_state("Connection is not open");
```

## Transport Layer

### Transport

Low-level transport layer for AMQP connections.

```rust
pub struct Transport {
    stream: TcpStream,
    read_buffer: BytesMut,
    write_buffer: BytesMut,
    // ... other fields
}

impl Transport {
    pub fn new(stream: TcpStream) -> Self;
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, AmqpError>;
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), AmqpError>;
    pub async fn flush(&mut self) -> Result<(), AmqpError>;
}
```

### Frame

AMQP protocol frame.

```rust
pub struct Frame {
    pub size: u32,
    pub doff: u8,
    pub type_: u8,
    pub channel: u16,
    pub payload: Vec<u8>,
}
```

## Best Practices

### Connection Management

1. **Always close connections**: Ensure connections are properly closed to free resources.
2. **Handle connection errors**: Implement proper error handling for connection failures.
3. **Use connection pooling**: For high-throughput applications, consider connection pooling.

### Message Handling

1. **Check message properties**: Always verify message properties before processing.
2. **Handle large messages**: For large messages, consider streaming or chunking.
3. **Use appropriate content types**: Set proper content types for better interoperability.

### Error Handling

1. **Use specific error types**: Handle specific error types rather than generic errors.
2. **Implement retry logic**: For transient errors, implement appropriate retry mechanisms.
3. **Log errors**: Always log errors for debugging and monitoring.

### Performance

1. **Reuse encoders/decoders**: Reuse encoder and decoder instances when possible.
2. **Batch operations**: Batch multiple operations for better performance.
3. **Monitor credit**: Keep track of sender/receiver credit to avoid blocking.

## Examples

### Complete Producer Example

```rust
use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection
    let mut connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("producer-app")
        .build();

    // Open connection
    connection.open().await?;

    // Create session
    let mut session = connection.create_session().await?;
    session.begin().await?;

    // Create sender
    let mut sender = LinkBuilder::new()
        .name("producer")
        .target("my-queue")
        .build_sender(session.id().to_string());

    sender.attach().await?;
    sender.add_credit(100);

    // Send messages
    for i in 0..10 {
        let message = Message::text(format!("Message {}", i))
            .with_message_id(format!("msg-{}", i))
            .with_subject("Test Message");
        
        let delivery_id = sender.send(message).await?;
        println!("Sent message {} with delivery ID {}", i, delivery_id);
    }

    // Clean up
    sender.detach().await?;
    session.end().await?;
    connection.close().await?;

    Ok(())
}
```

### Complete Consumer Example

```rust
use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection
    let mut connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("consumer-app")
        .build();

    // Open connection
    connection.open().await?;

    // Create session
    let mut session = connection.create_session().await?;
    session.begin().await?;

    // Create receiver
    let mut receiver = LinkBuilder::new()
        .name("consumer")
        .source("my-queue")
        .build_receiver(session.id().to_string());

    receiver.attach().await?;
    receiver.add_credit(100);

    println!("Waiting for messages...");

    // Receive messages
    loop {
        match receiver.receive().await {
            Ok(Some(message)) => {
                if let Some(text) = message.body_as_text() {
                    println!("Received: {}", text);
                }
                
                if let Some(id) = message.message_id_as_string() {
                    println!("Message ID: {}", id);
                }
            }
            Ok(None) => {
                // No message available
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }

    // Clean up
    receiver.detach().await?;
    session.end().await?;
    connection.close().await?;

    Ok(())
}
``` 
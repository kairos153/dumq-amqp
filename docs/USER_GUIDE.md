# dumq_amqp User Guide

## Introduction

dumq_amqp is a complete implementation of the AMQP 1.0 messaging protocol in Rust. This guide will help you get started with using dumq_amqp for building messaging applications.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Concepts](#basic-concepts)
- [Connection Management](#connection-management)
- [Sending Messages](#sending-messages)
- [Receiving Messages](#receiving-messages)
- [Error Handling](#error-handling)
- [Advanced Features](#advanced-features)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

Add dumq_amqp to your `Cargo.toml`:

```toml
[dependencies]
dumq_amqp = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

Here's a simple example that demonstrates the basic usage:

```rust
use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection
    let mut connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("my-app")
        .build();

    // Open the connection
    connection.open().await?;

    // Create a session
    let mut session = connection.create_session().await?;
    session.begin().await?;

    // Create a sender
    let mut sender = LinkBuilder::new()
        .name("my-sender")
        .target("my-queue")
        .build_sender(session.id().to_string());

    sender.attach().await?;
    sender.add_credit(10);

    // Send a message
    let message = Message::text("Hello, AMQP!");
    let delivery_id = sender.send(message).await?;
    println!("Message sent with delivery ID: {}", delivery_id);

    // Clean up
    sender.detach().await?;
    session.end().await?;
    connection.close().await?;

    Ok(())
}
```

## Basic Concepts

### AMQP 1.0 Architecture

AMQP 1.0 uses a hierarchical structure:

1. **Connection**: The top-level container for all communication
2. **Session**: Provides flow control and link management
3. **Link**: Represents a sender or receiver for a specific source/target
4. **Message**: The actual data being transmitted

### Connection

A connection represents a network connection to an AMQP broker. It can contain multiple sessions and is the foundation for all AMQP communication.

### Session

Sessions provide flow control and manage the lifecycle of links. Each session can contain multiple links (senders and receivers).

### Link

Links represent the actual communication channels for sending or receiving messages. There are two types:
- **Sender**: Sends messages to a target
- **Receiver**: Receives messages from a source

### Message

Messages are the containers for your data. They can contain:
- **Header**: Delivery-related information
- **Properties**: Application-level metadata
- **Body**: The actual content
- **Annotations**: Additional metadata

## Connection Management

### Creating a Connection

```rust
use dumq_amqp::connection::ConnectionBuilder;
use tokio::time::Duration;

let mut connection = ConnectionBuilder::new()
    .hostname("localhost")
    .port(5672)
    .timeout(Duration::from_secs(30))
    .max_frame_size(65536)
    .channel_max(1000)
    .idle_timeout(Duration::from_secs(60))
    .container_id("my-application")
    .build();
```

### Connection Configuration Options

- **hostname**: The broker hostname
- **port**: The broker port (default: 5672)
- **timeout**: Connection timeout
- **max_frame_size**: Maximum frame size in bytes
- **channel_max**: Maximum number of channels
- **idle_timeout**: Idle timeout for the connection
- **container_id**: Unique identifier for this connection

### Opening and Closing Connections

```rust
// Open the connection
connection.open().await?;

// Use the connection...

// Close the connection
connection.close().await?;
```

### Connection States

Connections go through several states:
- **Closed**: Initial state
- **Opening**: Connection establishment in progress
- **Open**: Connection is active and ready
- **Closing**: Connection termination in progress
- **Closed**: Connection is terminated

## Sending Messages

### Creating a Sender

```rust
use dumq_amqp::link::LinkBuilder;

let mut sender = LinkBuilder::new()
    .name("my-sender")
    .target("my-queue")
    .sender_settle_mode(SenderSettleMode::Settled)
    .receiver_settle_mode(ReceiverSettleMode::First)
    .build_sender(session.id().to_string());
```

### Sender Configuration

- **name**: Unique name for the sender
- **target**: The target address (queue/topic)
- **sender_settle_mode**: How the sender handles settlement
- **receiver_settle_mode**: How the receiver handles settlement

### Sending Messages

```rust
// Attach the sender
sender.attach().await?;

// Add credit (required for sending)
sender.add_credit(10);

// Create and send a message
let message = Message::text("Hello, World!")
    .with_message_id("msg-001")
    .with_subject("Test Message");

let delivery_id = sender.send(message).await?;
println!("Message sent with delivery ID: {}", delivery_id);
```

### Message Types

#### Text Messages

```rust
let text_message = Message::text("Hello, World!");
```

#### Binary Messages

```rust
let binary_message = Message::binary(b"Binary data");
```

#### Complex Messages

```rust
use dumq_amqp::types::{AmqpValue, AmqpSymbol};

let complex_message = Message::builder()
    .body(message::Body::Value(AmqpValue::String("Custom content".to_string())))
    .build()
    .with_message_id("msg-001")
    .with_subject("Test Message")
    .with_content_type(AmqpSymbol::from("text/plain"));
```

### Message Properties

You can set various message properties:

```rust
let message = Message::text("Hello, World!")
    .with_message_id("msg-001")
    .with_subject("Test Message")
    .with_content_type(AmqpSymbol::from("text/plain"));
```

## Receiving Messages

### Creating a Receiver

```rust
use dumq_amqp::link::LinkBuilder;

let mut receiver = LinkBuilder::new()
    .name("my-receiver")
    .source("my-queue")
    .sender_settle_mode(SenderSettleMode::Settled)
    .receiver_settle_mode(ReceiverSettleMode::First)
    .build_receiver(session.id().to_string());
```

### Receiving Messages

```rust
// Attach the receiver
receiver.attach().await?;

// Add credit (required for receiving)
receiver.add_credit(10);

// Receive messages
while let Ok(Some(message)) = receiver.receive().await {
    if let Some(text) = message.body_as_text() {
        println!("Received: {}", text);
    }
    
    if let Some(id) = message.message_id_as_string() {
        println!("Message ID: {}", id);
    }
}
```

### Message Processing

#### Accessing Message Content

```rust
// Get text content
if let Some(text) = message.body_as_text() {
    println!("Text: {}", text);
}

// Get binary content
if let Some(binary) = message.body_as_binary() {
    println!("Binary: {:?}", binary);
}

// Get message properties
if let Some(id) = message.message_id_as_string() {
    println!("Message ID: {}", id);
}

if let Some(props) = &message.properties {
    if let Some(subject) = &props.subject {
        println!("Subject: {}", subject);
    }
}
```

#### Message Acknowledgments

AMQP 1.0 uses a credit-based flow control system. When you receive a message, you need to manage credit appropriately:

```rust
// Add credit when you're ready to receive more messages
receiver.add_credit(1);
```

## Error Handling

### Understanding AMQP Errors

dumq_amqp provides comprehensive error types:

```rust
use dumq_amqp::error::{AmqpError, AmqpResult};

fn handle_amqp_operation() -> AmqpResult<()> {
    match some_operation() {
        Ok(result) => Ok(result),
        Err(AmqpError::Connection(msg)) => {
            eprintln!("Connection error: {}", msg);
            // Handle connection error
            Err(AmqpError::connection("Failed to connect"))
        }
        Err(AmqpError::Timeout(msg)) => {
            eprintln!("Timeout error: {}", msg);
            // Handle timeout error
            Err(AmqpError::timeout("Operation timed out"))
        }
        Err(e) => Err(e),
    }
}
```

### Common Error Types

- **Connection**: Network or connection-related errors
- **Session**: Session management errors
- **Link**: Sender/receiver link errors
- **Timeout**: Operation timeout errors
- **InvalidState**: State machine violations
- **Encoding/Decoding**: Data serialization errors

### Error Recovery Strategies

#### Connection Recovery

```rust
async fn connect_with_retry() -> Result<Connection, Box<dyn std::error::Error>> {
    let mut attempts = 0;
    let max_attempts = 3;
    
    while attempts < max_attempts {
        match ConnectionBuilder::new()
            .hostname("localhost")
            .port(5672)
            .build()
            .open()
            .await
        {
            Ok(connection) => return Ok(connection),
            Err(e) => {
                attempts += 1;
                eprintln!("Connection attempt {} failed: {}", attempts, e);
                if attempts < max_attempts {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
    
    Err("Failed to connect after maximum attempts".into())
}
```

#### Message Processing with Error Handling

```rust
async fn process_messages(receiver: &mut Receiver) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match receiver.receive().await {
            Ok(Some(message)) => {
                match process_message(message).await {
                    Ok(_) => {
                        // Message processed successfully
                        receiver.add_credit(1);
                    }
                    Err(e) => {
                        eprintln!("Failed to process message: {}", e);
                        // Decide whether to continue or break
                        receiver.add_credit(1);
                    }
                }
            }
            Ok(None) => {
                // No message available
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                return Err(e.into());
            }
        }
    }
}
```

## Advanced Features

### Message Annotations

You can add custom annotations to messages:

```rust
use dumq_amqp::types::{AmqpMap, AmqpSymbol, AmqpValue};
use std::collections::HashMap;

let mut annotations = HashMap::new();
annotations.insert(AmqpSymbol::from("x-custom"), AmqpValue::String("value".to_string()));
let amqp_map = AmqpMap::from(annotations);

let message = Message::builder()
    .message_annotations(amqp_map)
    .body(message::Body::Value(AmqpValue::String("Hello".to_string())))
    .build();
```

### Application Properties

Add application-specific properties:

```rust
use dumq_amqp::types::{AmqpMap, AmqpSymbol, AmqpValue};
use std::collections::HashMap;

let mut properties = HashMap::new();
properties.insert(AmqpSymbol::from("app-version"), AmqpValue::String("1.0.0".to_string()));
properties.insert(AmqpSymbol::from("user-id"), AmqpValue::Int(12345));
let amqp_map = AmqpMap::from(properties);

let message = Message::builder()
    .application_properties(amqp_map)
    .body(message::Body::Value(AmqpValue::String("Hello".to_string())))
    .build();
```

### Message Headers

Set message delivery headers:

```rust
use dumq_amqp::message::Header;

let header = Header {
    durable: Some(true),
    priority: Some(5),
    ttl: Some(60000), // 60 seconds
    first_acquirer: Some(false),
    delivery_count: Some(0),
};

let message = Message::builder()
    .header(header)
    .body(message::Body::Value(AmqpValue::String("Hello".to_string())))
    .build();
```

### Credit Management

Proper credit management is crucial for performance:

```rust
// For senders
sender.add_credit(100); // Add credit in batches

// For receivers
receiver.add_credit(10); // Add credit when ready to process

// Monitor credit levels
println!("Sender credit: {}", sender.credit());
println!("Receiver credit: {}", receiver.credit());
```

## Troubleshooting

### Common Issues

#### Connection Failures

**Problem**: Cannot connect to broker
**Solutions**:
- Check broker is running
- Verify hostname and port
- Check network connectivity
- Verify authentication credentials

#### Message Not Received

**Problem**: Messages are sent but not received
**Solutions**:
- Check source/target addresses
- Verify receiver has credit
- Check message routing
- Verify queue/topic exists

#### Performance Issues

**Problem**: Low throughput
**Solutions**:
- Increase credit levels
- Use connection pooling
- Batch operations
- Monitor resource usage

#### Memory Issues

**Problem**: High memory usage
**Solutions**:
- Process messages immediately
- Limit credit levels
- Use streaming for large messages
- Monitor message sizes

### Debugging

#### Enable Logging

```rust
use env_logger;

fn main() {
    env_logger::init();
    // Your AMQP code here
}
```

#### Check Connection State

```rust
println!("Connection state: {:?}", connection.state());
println!("Session state: {:?}", session.state());
println!("Sender state: {:?}", sender.state());
```

#### Monitor Credit

```rust
println!("Sender credit: {}", sender.credit());
println!("Receiver credit: {}", receiver.credit());
```

### Best Practices

1. **Always close connections**: Ensure proper cleanup
2. **Handle errors gracefully**: Implement retry logic
3. **Monitor credit levels**: Avoid blocking
4. **Use appropriate timeouts**: Prevent hanging operations
5. **Log important events**: For debugging and monitoring
6. **Test with real brokers**: Validate against actual AMQP brokers

### Performance Tips

1. **Reuse connections**: Don't create new connections for each operation
2. **Batch operations**: Group multiple operations when possible
3. **Use appropriate credit levels**: Balance between throughput and memory usage
4. **Monitor resource usage**: Keep track of memory and CPU usage
5. **Use async operations**: Don't block the event loop

## Examples

### Producer Application

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
    for i in 0..100 {
        let message = Message::text(format!("Message {}", i))
            .with_message_id(format!("msg-{}", i))
            .with_subject("Test Message");
        
        let delivery_id = sender.send(message).await?;
        println!("Sent message {} with delivery ID {}", i, delivery_id);
        
        // Add more credit if needed
        if sender.credit() < 10 {
            sender.add_credit(50);
        }
    }

    // Clean up
    sender.detach().await?;
    session.end().await?;
    connection.close().await?;

    Ok(())
}
```

### Consumer Application

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

    // Process messages
    let mut message_count = 0;
    loop {
        match receiver.receive().await {
            Ok(Some(message)) => {
                message_count += 1;
                
                if let Some(text) = message.body_as_text() {
                    println!("Received message {}: {}", message_count, text);
                }
                
                if let Some(id) = message.message_id_as_string() {
                    println!("Message ID: {}", id);
                }
                
                // Add more credit if needed
                if receiver.credit() < 10 {
                    receiver.add_credit(50);
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

This user guide should help you get started with dumq_amqp and build robust messaging applications. For more detailed information, refer to the API documentation. 
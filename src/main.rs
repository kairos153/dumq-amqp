use dumq_amqp::prelude::*;
use dumq_amqp::message::Body;
use dumq_amqp::types::{SenderSettleMode, ReceiverSettleMode};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("AMQP 1.0 Rust Library Example");
    println!("=============================");

    // Example 1: Create and encode/decode AMQP values
    println!("\n1. AMQP Value Encoding/Decoding Example:");
    example_encoding_decoding()?;

    // Example 2: Create AMQP messages
    println!("\n2. AMQP Message Creation Example:");
    example_message_creation()?;

    // Example 3: Connection and session management
    println!("\n3. Connection and Session Example:");
    example_connection_session().await?;

    // Example 4: Sender and receiver
    println!("\n4. Sender and Receiver Example:");
    example_sender_receiver().await?;

    Ok(())
}

fn example_encoding_decoding() -> Result<(), Box<dyn std::error::Error>> {
    use dumq_amqp::codec::{Encoder, Decoder};

    // Create some AMQP values
    let values = vec![
        AmqpValue::String("Hello, AMQP!".to_string()),
        AmqpValue::Int(42),
        AmqpValue::Boolean(true),
        AmqpValue::Double(3.14159),
        AmqpValue::Uuid(uuid::Uuid::new_v4()),
    ];

    for value in values {
        println!("  Original: {:?}", value);
        
        // Encode the value
        let mut encoder = Encoder::new();
        encoder.encode_value(&value)?;
        let encoded = encoder.finish();
        
        println!("  Encoded: {:?} bytes", encoded.len());
        
        // Decode the value
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value()?;
        
        println!("  Decoded: {:?}", decoded);
        println!("  Match: {}", value == decoded);
        println!();
    }

    Ok(())
}

fn example_message_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple text message
    let text_message = Message::text("Hello, World!");
    println!("  Text message: {:?}", text_message.body_as_text());

    // Create a message with properties
    let message_with_props = Message::builder()
        .body(Body::Value(AmqpValue::String("Custom message".to_string())))
        .build()
        .with_message_id("msg-001")
        .with_subject("Test Message")
        .with_content_type(AmqpSymbol::from("text/plain"));
    
    println!("  Message with properties:");
    println!("    ID: {:?}", message_with_props.message_id_as_string());
    println!("    Subject: {:?}", message_with_props.properties.as_ref().and_then(|p| p.subject.as_ref()));

    // Create a binary message
    let binary_data = b"Binary message data";
    let binary_message = Message::binary(binary_data);
    println!("  Binary message: {:?} bytes", binary_message.body_as_binary().map(|b| b.len()));

    Ok(())
}

async fn example_connection_session() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection configuration
    let connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("rust-amqp-example")
        .build();

    println!("  Connection created with ID: {}", connection.id());
    println!("  Connection state: {:?}", connection.state());

    // Note: This would fail in a real scenario without an AMQP broker
    // For demonstration purposes, we'll just show the structure
    println!("  Connection configuration ready");
    println!("  (Skipping actual connection to avoid errors without broker)");

    Ok(())
}

async fn example_sender_receiver() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sender configuration
    let sender_config = LinkBuilder::new()
        .name("test-sender")
        .target("test-queue")
        .sender_settle_mode(SenderSettleMode::Settled)
        .receiver_settle_mode(ReceiverSettleMode::First)
        .build_sender("test-session".to_string());

    println!("  Sender created with name: {}", sender_config.name());
    println!("  Sender state: {:?}", sender_config.state());

    // Create a receiver configuration
    let receiver_config = LinkBuilder::new()
        .name("test-receiver")
        .source("test-queue")
        .sender_settle_mode(SenderSettleMode::Settled)
        .receiver_settle_mode(ReceiverSettleMode::First)
        .build_receiver("test-session".to_string());

    println!("  Receiver created with name: {}", receiver_config.name());
    println!("  Receiver state: {:?}", receiver_config.state());

    // Simulate message sending and receiving
    let test_message = Message::text("Test message from Rust AMQP library");
    
    // Add credit to sender (simulated)
    let mut sender = sender_config;
    sender.add_credit(10);
    println!("  Sender credit: {}", sender.credit());

    // Simulate sending a message
    match sender.send(test_message.clone()).await {
        Ok(delivery_id) => println!("  Message sent with delivery ID: {}", delivery_id),
        Err(e) => println!("  Failed to send message: {}", e),
    }

    // Simulate receiving a message
    let mut receiver = receiver_config;
    receiver.add_credit(10);
    receiver.simulate_receive(test_message);
    
    match receiver.receive().await {
        Ok(Some(message)) => {
            println!("  Message received: {:?}", message.body_as_text());
            println!("  Delivery count: {}", receiver.delivery_count());
        }
        Ok(None) => println!("  No message available"),
        Err(e) => println!("  Failed to receive message: {}", e),
    }

    Ok(())
}

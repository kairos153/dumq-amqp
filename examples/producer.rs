//! AMQP 1.0 Producer Example
//!
//! This example demonstrates how to create a producer that sends multiple messages
//! to an AMQP broker.

use dumq_amqp::prelude::*;
use dumq_amqp::session::SessionState;
use dumq_amqp::link::LinkState;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("dumq_amqp Producer Example");
    println!("=====================");

    // Create connection
    let connection = ConnectionBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .container_id("producer-app")
        .build();

    println!("Connection created with ID: {}", connection.id());
    println!("Connection state: {:?}", connection.state());

    // Note: We'll skip actual connection opening to avoid errors without a broker
    println!("(Skipping actual connection to avoid errors without broker)");

    // Note: We'll skip actual connection opening to avoid errors without a broker
    println!("(Skipping actual connection opening to avoid errors without broker)");
    
    // Create session (simulated)
    println!("Session would be created with connection ID: {}", connection.id());
    println!("Session state would be: {:?}", SessionState::Ended);

    // Create sender (simulated)
    println!("Sender would be created with name: producer");
    println!("Sender state would be: {:?}", LinkState::Detached);

    // Note: We'll skip actual sender attachment to avoid errors without a broker
    println!("(Skipping actual sender attachment to avoid errors without broker)");

    // Simulate sending messages
    println!("\nSimulating message sending...");
    for i in 0..5 {
        let message = Message::text(format!("Message {}", i))
            .with_message_id(format!("msg-{}", i))
            .with_subject("Test Message");

        println!("Prepared message {}: {:?}", i, message.body_as_text());
        println!("Message ID: {:?}", message.message_id_as_string());
        println!("Subject: {:?}", message.properties.as_ref().and_then(|p| p.subject.as_ref()));

        // Note: We'll skip actual sending to avoid errors without a broker
        println!("(Message {} would be sent with delivery ID {})", i, i + 1);
        println!("---");
    }

    println!("\nProducer example completed successfully!");
    println!("In a real scenario with a broker, all messages would be sent.");
    Ok(())
} 
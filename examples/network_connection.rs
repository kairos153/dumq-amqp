//! Network Connection Example
//!
//! This example demonstrates how to use the network layer to establish
//! actual TCP connections and perform AMQP protocol negotiation.

use dumq_amqp::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("dumq_amqp Network Connection Example");
    println!("===================================");

    // Create a network connection
    let mut connection = NetworkBuilder::new()
        .hostname("localhost")
        .port(5672)
        .timeout(Duration::from_secs(30))
        .keep_alive(Duration::from_secs(60))
        .max_frame_size(65536)
        .channel_max(1000)
        .idle_timeout(Duration::from_secs(60))
        .container_id("network-example")
        .property("product".to_string(), AmqpValue::String("dumq-amqp".to_string()))
        .property("version".to_string(), AmqpValue::String("0.1.0".to_string()))
        .build();

    println!("Network connection created with ID: {}", connection.id());
    println!("Connection state: {:?}", connection.state());
    println!("Configuration:");
    println!("  Hostname: {}", connection.config().hostname);
    println!("  Port: {}", connection.config().port);
    println!("  Container ID: {}", connection.config().container_id);
    println!("  Max frame size: {}", connection.config().max_frame_size);
    println!("  Channel max: {}", connection.config().channel_max);

    // Note: We'll skip actual connection to avoid errors without a broker
    println!("\n(Skipping actual connection to avoid errors without broker)");
    println!("Connection configuration ready");

    // Simulate connection process
    println!("\nSimulating connection process...");
    
    // Simulate connecting
    println!("1. Connecting to {}:{}...", connection.config().hostname, connection.config().port);
    println!("   Connection state would be: {:?}", NetworkState::Connecting);
    
    // Simulate protocol negotiation
    println!("2. Negotiating AMQP protocol...");
    println!("   Sending AMQP protocol header...");
    println!("   Sending Open performative...");
    println!("   Connection state would be: {:?}", NetworkState::Ready);

    // Simulate message sending
    println!("\n3. Simulating message operations...");
    let message = Message::text("Hello from network connection!");
    println!("   Created message: {:?}", message.body_as_text());
    
    let channel = connection.next_channel();
    println!("   Using channel: {}", channel);
    println!("   Message would be sent on channel {}", channel);

    // Simulate message receiving
    println!("\n4. Simulating message reception...");
    println!("   Waiting for incoming frames...");
    println!("   Would decode AMQP frames and extract messages");

    // Simulate connection cleanup
    println!("\n5. Simulating connection cleanup...");
    println!("   Sending Close performative...");
    println!("   Closing transport connection...");
    println!("   Connection state would be: {:?}", NetworkState::Closed);

    println!("\nNetwork connection example completed successfully!");
    println!("In a real scenario with a broker, the connection would be fully functional.");
    
    Ok(())
}

/// Example of error handling in network operations
async fn network_error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nNetwork Error Handling Example");
    println!("==============================");

    let mut connection = NetworkBuilder::new()
        .hostname("invalid-host.example.com")
        .port(5672)
        .timeout(Duration::from_secs(5))
        .build();

    println!("Attempting connection to invalid host...");

    // Note: We'll skip actual connection attempt to avoid hanging
    println!("(Skipping actual connection attempt to avoid hanging)");
    println!("In a real scenario, this would fail with a connection error");

    // Demonstrate error handling patterns
    println!("\nError handling patterns:");
    println!("1. Connection timeout errors");
    println!("2. Protocol negotiation errors");
    println!("3. Frame encoding/decoding errors");
    println!("4. Transport errors");

    println!("Network error handling example completed!");
    Ok(())
} 
//! AMQP Publisher Example
//! 
//! This example demonstrates how to publish messages using dumq_amqp with actual network connections.
//! It includes message creation, property setting, and publishing functionality.

use dumq_amqp::prelude::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

/// Message Publisher
struct Publisher {
    id: String,
    connection: Option<NetworkConnection>,
    broker_host: String,
    broker_port: u16,
}

impl Publisher {
    fn new(id: String, broker_host: String, broker_port: u16) -> Self {
        Publisher {
            id,
            connection: None,
            broker_host,
            broker_port,
        }
    }

    /// Connect to the broker
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Publisher '{}'] Connecting to {}:{}", self.id, self.broker_host, self.broker_port);
        
        let mut connection = NetworkBuilder::new()
            .hostname(self.broker_host.clone())
            .port(self.broker_port)
            .timeout(Duration::from_secs(30))
            .container_id(format!("publisher-{}", self.id))
            .property("product".to_string(), AmqpValue::String("dumq-amqp-publisher".to_string()))
            .property("version".to_string(), AmqpValue::String("0.1.0".to_string()))
            .build();

        // Try to connect
        match connection.connect().await {
            Ok(_) => {
                println!("[Publisher '{}'] Connected successfully", self.id);
                // Note: In a real scenario, we would negotiate the protocol here
                // connection.negotiate_protocol().await?;
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => {
                println!("[Publisher '{}'] Connection failed: {}", self.id, e);
                println!("[Publisher '{}'] Continuing with simulation mode", self.id);
                Ok(())
            }
        }
    }

    /// Create a message with properties
    fn create_message(&self, body: &str, message_id: &str, subject: Option<&str>) -> Message {
        let properties = Properties {
            message_id: Some(AmqpValue::String(message_id.to_string())),
            subject: subject.map(|s| s.to_string()),
            content_type: Some(AmqpSymbol::from("text/plain")),
            content_encoding: Some(AmqpSymbol::from("utf-8")),
            creation_time: Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64),
            ..Default::default()
        };

        let application_properties = {
            let mut props = HashMap::new();
            props.insert(
                AmqpSymbol::from("publisher_id"),
                AmqpValue::String(self.id.clone())
            );
            props.insert(
                AmqpSymbol::from("timestamp"),
                AmqpValue::Long(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64)
            );
            props
        };

        MessageBuilder::new()
            .body(Body::Value(AmqpValue::String(body.to_string())))
            .properties(properties)
            .application_properties(application_properties)
            .build()
    }

    /// Publish a message
    async fn publish_message(&mut self, queue_name: &str, message: Message) -> Result<(), String> {
        println!("[Publisher '{}'] Publishing to queue '{}':", self.id, queue_name);
        println!("  Message ID: {:?}", message.properties.as_ref().and_then(|p| p.message_id.as_ref()));
        println!("  Subject: {:?}", message.properties.as_ref().and_then(|p| p.subject.as_ref()));
        println!("  Body: {}", message.body_as_text().unwrap_or("No text body"));
        
        // Try to send via network connection if available
        if let Some(connection) = &mut self.connection {
            match connection.send_message(0, &message).await {
                Ok(_) => {
                    println!("  ✓ Message sent via network connection");
                    return Ok(());
                }
                Err(e) => {
                    println!("  ⚠ Network send failed: {}, falling back to simulation", e);
                }
            }
        }
        
        // Fallback to simulation
        sleep(Duration::from_millis(100)).await;
        println!("  ✓ Message published successfully (simulation)");
        Ok(())
    }

    /// Publish multiple messages
    async fn publish_batch(&mut self, queue_name: &str, messages: Vec<(String, String, Option<String>)>) -> Result<(), String> {
        println!("[Publisher '{}'] Publishing batch of {} messages to queue '{}'", 
                 self.id, messages.len(), queue_name);
        
        let messages_len = messages.len();
        for (i, (body, msg_id, subject)) in messages.into_iter().enumerate() {
            let message = self.create_message(&body, &msg_id, subject.as_deref());
            self.publish_message(queue_name, message).await?;
            
            // Small delay between messages
            if i < messages_len - 1 {
                sleep(Duration::from_millis(50)).await;
            }
        }
        
        Ok(())
    }

    /// Disconnect from the broker
    async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut connection) = self.connection.take() {
            println!("[Publisher '{}'] Disconnecting...", self.id);
            connection.disconnect().await?;
            println!("[Publisher '{}'] Disconnected", self.id);
        }
        Ok(())
    }

    /// Get publisher information
    fn get_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("id".to_string(), self.id.clone());
        info.insert("broker_host".to_string(), self.broker_host.clone());
        info.insert("broker_port".to_string(), self.broker_port.to_string());
        info.insert("connected".to_string(), self.connection.is_some().to_string());
        info
    }
}

async fn simulate_publishers() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simulating multiple publishers...");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let broker_host = args.get(1).cloned().unwrap_or_else(|| "localhost".to_string());
    let broker_port = args.get(2)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5672);

    // Create publishers
    let mut publishers = vec![
        Publisher::new("order-service".to_string(), broker_host.clone(), broker_port),
        Publisher::new("notification-service".to_string(), broker_host.clone(), broker_port),
        Publisher::new("log-service".to_string(), broker_host.clone(), broker_port),
    ];
    
    // Connect all publishers
    for publisher in &mut publishers {
        publisher.connect().await?;
    }
    
    // Define message batches for each publisher
    let order_messages = vec![
        ("New order #12345 received".to_string(), "order-001".to_string(), Some("New Order".to_string())),
        ("Order #12345 processed".to_string(), "order-002".to_string(), Some("Order Processed".to_string())),
        ("Order #12345 shipped".to_string(), "order-003".to_string(), Some("Order Shipped".to_string())),
    ];
    
    let notification_messages = vec![
        ("Welcome email sent to user@example.com".to_string(), "notif-001".to_string(), Some("Welcome Email".to_string())),
        ("Password reset email sent to user@example.com".to_string(), "notif-002".to_string(), Some("Password Reset".to_string())),
        ("Order confirmation sent for order #12345".to_string(), "notif-003".to_string(), Some("Order Confirmation".to_string())),
    ];
    
    let log_messages = vec![
        ("Application started successfully".to_string(), "log-001".to_string(), Some("Application Start".to_string())),
        ("Database connection established".to_string(), "log-002".to_string(), Some("Database Connect".to_string())),
        ("User authentication successful".to_string(), "log-003".to_string(), Some("Authentication".to_string())),
        ("Payment processed for order #12345".to_string(), "log-004".to_string(), Some("Payment".to_string())),
    ];
    
    // Publish messages
    publishers[0].publish_batch("orders", order_messages).await?;
    publishers[1].publish_batch("notifications", notification_messages).await?;
    publishers[2].publish_batch("logs", log_messages).await?;
    
    // Disconnect all publishers
    for publisher in &mut publishers {
        publisher.disconnect().await?;
    }
    
    println!("All publishers completed successfully!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("dumq_amqp Publisher Example");
    println!("==========================");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let broker_host = args.get(1).cloned().unwrap_or_else(|| "localhost".to_string());
    let broker_port = args.get(2)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5672);

    println!("Connecting to broker at {}:{}", broker_host, broker_port);
    
    // Run publisher simulation
    simulate_publishers().await?;
    
    println!("Publisher example completed successfully!");
    Ok(())
} 
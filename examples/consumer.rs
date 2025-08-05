//! AMQP Consumer Example
//! 
//! This example demonstrates how to consume messages using dumq_amqp with actual network connections.
//! It includes message consumption, processing, and acknowledgment functionality.

use dumq_amqp::prelude::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

/// Message Consumer
struct Consumer {
    id: String,
    queue_name: String,
    connection: Option<NetworkConnection>,
    broker_host: String,
    broker_port: u16,
    processed_count: u32,
    error_count: u32,
}

impl Consumer {
    fn new(id: String, queue_name: String, broker_host: String, broker_port: u16) -> Self {
        Consumer {
            id,
            queue_name,
            connection: None,
            broker_host,
            broker_port,
            processed_count: 0,
            error_count: 0,
        }
    }

    /// Connect to the broker
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Consumer '{}'] Connecting to {}:{}", self.id, self.broker_host, self.broker_port);
        
        let mut connection = NetworkBuilder::new()
            .hostname(self.broker_host.clone())
            .port(self.broker_port)
            .timeout(Duration::from_secs(30))
            .container_id(format!("consumer-{}", self.id))
            .property("product".to_string(), AmqpValue::String("dumq-amqp-consumer".to_string()))
            .property("version".to_string(), AmqpValue::String("0.1.0".to_string()))
            .build();

        // Try to connect
        match connection.connect().await {
            Ok(_) => {
                println!("[Consumer '{}'] Connected successfully", self.id);
                // Note: In a real scenario, we would negotiate the protocol here
                // connection.negotiate_protocol().await?;
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => {
                println!("[Consumer '{}'] Connection failed: {}", self.id, e);
                println!("[Consumer '{}'] Continuing with simulation mode", self.id);
                Ok(())
            }
        }
    }

    /// Consume a message
    async fn consume_message(&mut self, message: &Message) -> Result<(), String> {
        println!("[Consumer '{}'] Consuming from queue '{}':", self.id, self.queue_name);
        
        // Extract message information
        let message_id = message.properties.as_ref()
            .and_then(|p| p.message_id.as_ref())
            .and_then(|id| match id {
                AmqpValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "unknown".to_string());
            
        let subject = message.properties.as_ref()
            .and_then(|p| p.subject.as_ref())
            .cloned()
            .unwrap_or_else(|| "No subject".to_string());
            
        let body = message.body_as_text().unwrap_or("No text body");
        
        println!("  Message ID: {}", message_id);
        println!("  Subject: {}", subject);
        println!("  Body: {}", body);
        
        // Simulate message processing
        match self.process_message(&message_id, &subject, body).await {
            Ok(_) => {
                self.processed_count += 1;
                println!("  ✓ Message processed successfully");
                self.acknowledge_message(&message_id).await?;
            }
            Err(e) => {
                self.error_count += 1;
                println!("  ✗ Message processing failed: {}", e);
                self.reject_message(&message_id, &e).await?;
            }
        }
        
        Ok(())
    }

    /// Process a message based on its type
    async fn process_message(&self, message_id: &str, subject: &str, body: &str) -> Result<(), String> {
        // Simulate processing delay
        sleep(Duration::from_millis(200)).await;
        
        match subject {
            "New Order" => self.process_order(message_id, body).await,
            "Order Processed" => self.process_order_update(message_id, body).await,
            "Order Shipped" => self.process_shipping(message_id, body).await,
            "Welcome Email" => self.process_welcome_email(message_id, body).await,
            "Password Reset" => self.process_password_reset(message_id, body).await,
            "Order Confirmation" => self.process_order_confirmation(message_id, body).await,
            "Application Start" => self.process_log(message_id, body).await,
            "Database Connect" => self.process_log(message_id, body).await,
            "Authentication" => self.process_log(message_id, body).await,
            "Payment" => self.process_log(message_id, body).await,
            _ => self.process_unknown_message(message_id, subject, body).await,
        }
    }

    /// Process order messages
    async fn process_order(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing order: {}", body);
        // Simulate order processing
        sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    /// Process order update messages
    async fn process_order_update(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing order update: {}", body);
        // Simulate order update processing
        sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    /// Process shipping messages
    async fn process_shipping(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing shipping: {}", body);
        // Simulate shipping processing
        sleep(Duration::from_millis(75)).await;
        Ok(())
    }

    /// Process welcome email messages
    async fn process_welcome_email(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing welcome email: {}", body);
        // Simulate email processing
        sleep(Duration::from_millis(25)).await;
        Ok(())
    }

    /// Process password reset messages
    async fn process_password_reset(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing password reset: {}", body);
        // Simulate password reset processing
        sleep(Duration::from_millis(30)).await;
        Ok(())
    }

    /// Process order confirmation messages
    async fn process_order_confirmation(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing order confirmation: {}", body);
        // Simulate order confirmation processing
        sleep(Duration::from_millis(40)).await;
        Ok(())
    }

    /// Process log messages
    async fn process_log(&self, _message_id: &str, body: &str) -> Result<(), String> {
        println!("    Processing log: {}", body);
        // Simulate log processing
        sleep(Duration::from_millis(10)).await;
        Ok(())
    }

    /// Process unknown message types
    async fn process_unknown_message(&self, _message_id: &str, subject: &str, _body: &str) -> Result<(), String> {
        println!("    Processing unknown message type: {}", subject);
        // Simulate unknown message processing
        sleep(Duration::from_millis(20)).await;
        Ok(())
    }

    /// Acknowledge a message
    async fn acknowledge_message(&self, message_id: &str) -> Result<(), String> {
        println!("    Acknowledging message: {}", message_id);
        // In a real implementation, this would send an acknowledgment to the broker
        Ok(())
    }

    /// Reject a message
    async fn reject_message(&self, message_id: &str, reason: &str) -> Result<(), String> {
        println!("    Rejecting message: {} - Reason: {}", message_id, reason);
        // In a real implementation, this would send a rejection to the broker
        Ok(())
    }

    /// Get consumer statistics
    fn get_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("processed_count".to_string(), self.processed_count);
        stats.insert("error_count".to_string(), self.error_count);
        stats
    }

    /// Get consumer information
    fn get_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("id".to_string(), self.id.clone());
        info.insert("queue_name".to_string(), self.queue_name.clone());
        info.insert("broker_host".to_string(), self.broker_host.clone());
        info.insert("broker_port".to_string(), self.broker_port.to_string());
        info.insert("connected".to_string(), self.connection.is_some().to_string());
        info
    }

    /// Disconnect from the broker
    async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut connection) = self.connection.take() {
            println!("[Consumer '{}'] Disconnecting...", self.id);
            connection.disconnect().await?;
            println!("[Consumer '{}'] Disconnected", self.id);
        }
        Ok(())
    }
}

async fn simulate_consumers() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simulating multiple consumers...");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let broker_host = args.get(1).cloned().unwrap_or_else(|| "localhost".to_string());
    let broker_port = args.get(2)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5672);
    
    // Create consumers
    let mut consumers = vec![
        Consumer::new("order-processor".to_string(), "orders".to_string(), broker_host.clone(), broker_port),
        Consumer::new("email-service".to_string(), "notifications".to_string(), broker_host.clone(), broker_port),
        Consumer::new("log-processor".to_string(), "logs".to_string(), broker_host.clone(), broker_port),
    ];
    
    // Connect all consumers
    for consumer in &mut consumers {
        consumer.connect().await?;
    }
    
    // Create test messages for each consumer
    let order_messages = vec![
        create_test_message("order-001", "New Order", "New order #12345 received"),
        create_test_message("order-002", "Order Processed", "Order #12345 processed"),
        create_test_message("order-003", "Order Shipped", "Order #12345 shipped"),
    ];
    
    let notification_messages = vec![
        create_test_message("notif-001", "Welcome Email", "Welcome email sent to user@example.com"),
        create_test_message("notif-002", "Password Reset", "Password reset email sent to user@example.com"),
        create_test_message("notif-003", "Order Confirmation", "Order confirmation sent for order #12345"),
    ];
    
    let log_messages = vec![
        create_test_message("log-001", "Application Start", "Application started successfully"),
        create_test_message("log-002", "Database Connect", "Database connection established"),
        create_test_message("log-003", "Authentication", "User authentication successful"),
        create_test_message("log-004", "Payment", "Payment processed for order #12345"),
    ];
    
    // Process messages for each consumer
    println!("\nProcessing order messages:");
    for message in order_messages {
        consumers[0].consume_message(&message).await?;
    }
    
    println!("\nProcessing notification messages:");
    for message in notification_messages {
        consumers[1].consume_message(&message).await?;
    }
    
    println!("\nProcessing log messages:");
    for message in log_messages {
        consumers[2].consume_message(&message).await?;
    }
    
    // Show statistics
    println!("\nConsumer Statistics:");
    for consumer in &consumers {
        let stats = consumer.get_stats();
        let info = consumer.get_info();
        println!("  Consumer: {} (Queue: {})", info["id"], info["queue_name"]);
        println!("    Processed: {}, Errors: {}", stats["processed_count"], stats["error_count"]);
        println!("    Connected: {}", info["connected"]);
    }
    
    // Disconnect all consumers
    for consumer in &mut consumers {
        consumer.disconnect().await?;
    }
    
    println!("All consumers completed successfully!");
    Ok(())
}

fn create_test_message(message_id: &str, subject: &str, body: &str) -> Message {
    let properties = Properties {
        message_id: Some(AmqpValue::String(message_id.to_string())),
        subject: Some(subject.to_string()),
        content_type: Some(AmqpSymbol::from("text/plain")),
        content_encoding: Some(AmqpSymbol::from("utf-8")),
        creation_time: Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64),
        ..Default::default()
    };

    MessageBuilder::new()
        .body(Body::Value(AmqpValue::String(body.to_string())))
        .properties(properties)
        .build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("dumq_amqp Consumer Example");
    println!("=========================");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let broker_host = args.get(1).cloned().unwrap_or_else(|| "localhost".to_string());
    let broker_port = args.get(2)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5672);

    println!("Connecting to broker at {}:{}", broker_host, broker_port);
    
    // Run consumer simulation
    simulate_consumers().await?;
    
    println!("Consumer example completed successfully!");
    Ok(())
} 
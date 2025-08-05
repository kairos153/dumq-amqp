//! Simple AMQP Broker Example
//! 
//! This example demonstrates a basic AMQP broker implementation using dumq_amqp.
//! It includes message queuing, connection management, and message routing with actual network connections.

use dumq_amqp::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

/// Simple message queue
#[derive(Debug, Clone)]
struct QueueMessage {
    id: String,
    body: String,
    properties: Option<Properties>,
    timestamp: std::time::SystemTime,
}

/// Queue for storing messages
#[derive(Debug)]
struct Queue {
    name: String,
    messages: Vec<QueueMessage>,
    consumers: Vec<String>, // Consumer IDs
}

impl Queue {
    fn new(name: String) -> Self {
        Queue {
            name,
            messages: Vec::new(),
            consumers: Vec::new(),
        }
    }

    fn publish(&mut self, message: QueueMessage) {
        println!("  [Queue '{}'] Publishing message: {}", self.name, message.id);
        self.messages.push(message);
    }

    fn consume(&mut self, consumer_id: &str) -> Option<QueueMessage> {
        if let Some(message) = self.messages.pop() {
            println!("  [Queue '{}'] Consuming message: {} by consumer: {}", 
                     self.name, message.id, consumer_id);
            Some(message)
        } else {
            None
        }
    }

    fn add_consumer(&mut self, consumer_id: String) {
        if !self.consumers.contains(&consumer_id) {
            let consumer_id_clone = consumer_id.clone();
            self.consumers.push(consumer_id);
            println!("  [Queue '{}'] Added consumer: {}", self.name, consumer_id_clone);
        }
    }

    fn remove_consumer(&mut self, consumer_id: &str) {
        self.consumers.retain(|id| id != consumer_id);
        println!("  [Queue '{}'] Removed consumer: {}", self.name, consumer_id);
    }
}

/// Simple AMQP Broker
#[derive(Debug)]
struct SimpleBroker {
    queues: HashMap<String, Queue>,
    connections: HashMap<String, ConnectionInfo>,
    listener: Option<TcpListener>,
    broker_port: u16,
}

#[derive(Debug)]
struct ConnectionInfo {
    id: String,
    hostname: String,
    port: u16,
    connected_at: std::time::SystemTime,
}

impl SimpleBroker {
    fn new(port: u16) -> Self {
        SimpleBroker {
            queues: HashMap::new(),
            connections: HashMap::new(),
            listener: None,
            broker_port: port,
        }
    }

    /// Start the broker server
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.broker_port);
        let listener = TcpListener::bind(&addr).await?;
        self.listener = Some(listener);
        
        println!("[Broker] Started on {}", addr);
        println!("[Broker] Waiting for connections...");
        
        Ok(())
    }

    /// Accept and handle client connections
    async fn accept_connections(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(listener) = &self.listener {
            loop {
                match listener.accept().await {
                    Ok((socket, addr)) => {
                        println!("[Broker] New connection from: {}", addr);
                        
                        // Handle connection in a separate task
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_connection_task(socket, addr).await {
                                eprintln!("[Broker] Error handling connection: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("[Broker] Error accepting connection: {}", e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle a client connection (static method to avoid borrow checker issues)
    async fn handle_connection_task(socket: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let connection_id = format!("conn-{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
        println!("[Broker] Handling connection: {}", connection_id);
        
        // For now, just simulate handling the connection
        // In a real implementation, this would handle AMQP protocol negotiation
        sleep(Duration::from_millis(100)).await;
        
        println!("[Broker] Connection {} handled successfully", connection_id);
        
        Ok(())
    }

    fn create_queue(&mut self, name: String) -> Result<(), String> {
        if self.queues.contains_key(&name) {
            return Err(format!("Queue '{}' already exists", name));
        }
        
        self.queues.insert(name.clone(), Queue::new(name.clone()));
        println!("[Broker] Created queue: {}", name);
        Ok(())
    }

    fn delete_queue(&mut self, name: &str) -> Result<(), String> {
        if let Some(_) = self.queues.remove(name) {
            println!("[Broker] Deleted queue: {}", name);
            Ok(())
        } else {
            Err(format!("Queue '{}' not found", name))
        }
    }

    fn publish_message(&mut self, queue_name: &str, message: QueueMessage) -> Result<(), String> {
        if let Some(queue) = self.queues.get_mut(queue_name) {
            queue.publish(message);
            Ok(())
        } else {
            Err(format!("Queue '{}' not found", queue_name))
        }
    }

    fn consume_message(&mut self, queue_name: &str, consumer_id: &str) -> Result<Option<QueueMessage>, String> {
        if let Some(queue) = self.queues.get_mut(queue_name) {
            Ok(queue.consume(consumer_id))
        } else {
            Err(format!("Queue '{}' not found", queue_name))
        }
    }

    fn add_connection(&mut self, connection_id: String, hostname: String, port: u16) {
        let connection_info = ConnectionInfo {
            id: connection_id.clone(),
            hostname,
            port,
            connected_at: std::time::SystemTime::now(),
        };
        
        self.connections.insert(connection_id.clone(), connection_info);
        println!("[Broker] Added connection: {}", connection_id);
    }

    fn remove_connection(&mut self, connection_id: &str) {
        if let Some(_) = self.connections.remove(connection_id) {
            println!("[Broker] Removed connection: {}", connection_id);
        }
    }

    fn list_queues(&self) -> Vec<String> {
        self.queues.keys().cloned().collect()
    }

    fn list_connections(&self) -> Vec<String> {
        self.connections.keys().cloned().collect()
    }

    fn get_queue_info(&self, queue_name: &str) -> Option<(usize, usize)> {
        self.queues.get(queue_name).map(|queue| (queue.messages.len(), queue.consumers.len()))
    }

    /// Stop the broker
    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Broker] Stopping broker...");
        self.listener = None;
        println!("[Broker] Broker stopped");
        Ok(())
    }
}

async fn simulate_broker_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simulating broker operations...");
    
    // Create broker
    let mut broker = SimpleBroker::new(5672);
    
    // Create some queues
    broker.create_queue("orders".to_string())?;
    broker.create_queue("notifications".to_string())?;
    broker.create_queue("logs".to_string())?;
    
    // Simulate some messages
    let test_message = QueueMessage {
        id: "msg-001".to_string(),
        body: "Test message".to_string(),
        properties: None,
        timestamp: std::time::SystemTime::now(),
    };
    
    broker.publish_message("orders", test_message)?;
    
    // List queues
    println!("Queues: {:?}", broker.list_queues());
    
    // Get queue info
    if let Some((msg_count, consumer_count)) = broker.get_queue_info("orders") {
        println!("Orders queue: {} messages, {} consumers", msg_count, consumer_count);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("dumq_amqp Simple Broker Example");
    println!("==============================");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let port = args.get(1)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5672);

    // Create and start broker
    let mut broker = SimpleBroker::new(port);
    
    // Start the broker server
    broker.start().await?;
    
    // Simulate some operations
    simulate_broker_operations().await?;
    
    // Start accepting connections
    println!("[Broker] Starting to accept connections...");
    broker.accept_connections().await?;
    
    Ok(())
} 
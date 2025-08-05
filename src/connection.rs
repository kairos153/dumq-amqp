//! AMQP 1.0 Connection Management
//!
//! This module provides connection management for AMQP 1.0, including connection
//! establishment, configuration, and lifecycle management.
//!
//! # Overview
//!
//! Connections in AMQP 1.0 represent the top-level container for all communication
//! between peers. A connection can contain multiple sessions, and each session can
//! contain multiple links (senders and receivers).
//!
//! # Connection Lifecycle
//!
//! 1. **Closed**: Initial state
//! 2. **Opening**: Connection establishment in progress
//! 3. **Open**: Connection is active and ready for use
//! 4. **Closing**: Connection termination in progress
//! 5. **Closed**: Connection is terminated
//!
//! # Examples
//!
//! ## Basic Connection Usage
//!
//! ```rust
//! use dumq_amqp::connection::{Connection, ConnectionBuilder};
//! use tokio::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a connection
//!     let mut connection = ConnectionBuilder::new()
//!         .hostname("localhost")
//!         .port(5672)
//!         .timeout(Duration::from_secs(30))
//!         .container_id("my-app")
//!         .build();
//!
//!     // Open the connection
//!     connection.open().await?;
//!
//!     // Use the connection...
//!
//!     // Close the connection
//!     connection.close().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Connection Configuration
//!
//! ```rust
//! use dumq_amqp::connection::ConnectionBuilder;
//! use dumq_amqp::types::AmqpValue;
//! use tokio::time::Duration;
//!
//! let connection = ConnectionBuilder::new()
//!     .hostname("my-broker.example.com")
//!     .port(5672)
//!     .timeout(Duration::from_secs(30))
//!     .max_frame_size(65536)
//!     .channel_max(1000)
//!     .idle_timeout(Duration::from_secs(60))
//!     .container_id("my-application")
//!     .property("product".to_string(), AmqpValue::String("MyApp".to_string()))
//!     .build();
//! ```

use crate::{AmqpError, AmqpResult, AmqpValue};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

/// AMQP 1.0 Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Connection is being established
    Opening,
    /// Connection is open and ready
    Open,
    /// Connection is being closed
    Closing,
    /// Connection is closed
    Closed,
    /// Connection is in error state
    Error(String),
}

/// AMQP 1.0 Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Connection hostname
    pub hostname: String,
    /// Connection port
    pub port: u16,
    /// Connection timeout
    pub timeout: Duration,
    /// Maximum frame size
    pub max_frame_size: u32,
    /// Channel maximum
    pub channel_max: u16,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Container ID
    pub container_id: String,
    /// Connection properties
    pub properties: HashMap<String, AmqpValue>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        ConnectionConfig {
            hostname: "localhost".to_string(),
            port: 5672,
            timeout: Duration::from_secs(30),
            max_frame_size: 65536,
            channel_max: 0,
            idle_timeout: Duration::from_secs(0),
            container_id: Uuid::new_v4().to_string(),
            properties: HashMap::new(),
        }
    }
}

/// AMQP 1.0 Connection
pub struct Connection {
    /// Connection state
    state: ConnectionState,
    /// Connection configuration
    config: ConnectionConfig,
    /// TCP stream
    stream: Option<TcpStream>,
    /// Connection ID
    id: String,
    /// Next channel number
    next_channel: u16,
    /// Sessions
    sessions: HashMap<u16, Session>,
}

impl Connection {
    /// Create a new connection
    pub fn new(config: ConnectionConfig) -> Self {
        Connection {
            state: ConnectionState::Closed,
            config,
            stream: None,
            id: Uuid::new_v4().to_string(),
            next_channel: 0,
            sessions: HashMap::new(),
        }
    }

    /// Open the connection
    pub async fn open(&mut self) -> AmqpResult<()> {
        if self.state != ConnectionState::Closed {
            return Err(AmqpError::invalid_state("Connection is not in closed state"));
        }

        self.state = ConnectionState::Opening;

        // Connect to the server
        let addr = format!("{}:{}", self.config.hostname, self.config.port);
        let stream = timeout(self.config.timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| AmqpError::timeout("Connection timeout"))?
            .map_err(|e| AmqpError::connection(format!("Failed to connect: {}", e)))?;

        self.stream = Some(stream);
        self.state = ConnectionState::Open;

        // Send AMQP protocol header
        self.send_protocol_header().await?;

        // Send Open performative
        self.send_open().await?;

        Ok(())
    }

    /// Close the connection
    pub async fn close(&mut self) -> AmqpResult<()> {
        if self.state != ConnectionState::Open {
            return Err(AmqpError::invalid_state("Connection is not open"));
        }

        self.state = ConnectionState::Closing;

        // Close all sessions
        for session in self.sessions.values_mut() {
            session.close().await?;
        }
        self.sessions.clear();

        // Send Close performative
        self.send_close().await?;

        // Close TCP connection
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await
                .map_err(|e| AmqpError::connection(format!("Failed to close connection: {}", e)))?;
        }

        self.state = ConnectionState::Closed;
        Ok(())
    }

    /// Create a new session
    pub async fn create_session(&mut self) -> AmqpResult<Session> {
        if self.state != ConnectionState::Open {
            return Err(AmqpError::invalid_state("Connection is not open"));
        }

        let channel = self.next_channel;
        self.next_channel += 1;

        let session = Session::new(channel, self.id.clone());
        self.sessions.insert(channel, session.clone());

        Ok(session)
    }

    /// Get connection state
    pub fn state(&self) -> &ConnectionState {
        &self.state
    }

    /// Get connection ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Send AMQP protocol header
    async fn send_protocol_header(&self) -> AmqpResult<()> {
        // AMQP 1.0 protocol header: "AMQP\x00\x01\x00\x00"
        let header = [0x41, 0x4D, 0x51, 0x50, 0x00, 0x01, 0x00, 0x00];
        
        if let Some(stream) = &self.stream {
            stream.writable().await
                .map_err(|e| AmqpError::connection(format!("Stream not writable: {}", e)))?;
            
            stream.try_write(&header)
                .map_err(|e| AmqpError::connection(format!("Failed to write protocol header: {}", e)))?;
        }

        Ok(())
    }

    /// Send Open performative
    async fn send_open(&self) -> AmqpResult<()> {
        // This is a simplified implementation
        // In a real implementation, you would encode the Open performative properly
        log::debug!("Sending Open performative");
        Ok(())
    }

    /// Send Close performative
    async fn send_close(&self) -> AmqpResult<()> {
        // This is a simplified implementation
        // In a real implementation, you would encode the Close performative properly
        log::debug!("Sending Close performative");
        Ok(())
    }
}

/// Connection Builder for constructing AMQP 1.0 connections
#[derive(Debug, Clone)]
pub struct ConnectionBuilder {
    config: ConnectionConfig,
}

impl ConnectionBuilder {
    /// Create a new connection builder
    pub fn new() -> Self {
        ConnectionBuilder {
            config: ConnectionConfig::default(),
        }
    }

    /// Set the hostname
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.config.hostname = hostname.into();
        self
    }

    /// Set the port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set the connection timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the maximum frame size
    pub fn max_frame_size(mut self, max_frame_size: u32) -> Self {
        self.config.max_frame_size = max_frame_size;
        self
    }

    /// Set the channel maximum
    pub fn channel_max(mut self, channel_max: u16) -> Self {
        self.config.channel_max = channel_max;
        self
    }

    /// Set the idle timeout
    pub fn idle_timeout(mut self, idle_timeout: Duration) -> Self {
        self.config.idle_timeout = idle_timeout;
        self
    }

    /// Set the container ID
    pub fn container_id(mut self, container_id: impl Into<String>) -> Self {
        self.config.container_id = container_id.into();
        self
    }

    /// Add a connection property
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self {
        self.config.properties.insert(key.into(), value);
        self
    }

    /// Build the connection
    pub fn build(self) -> Connection {
        Connection::new(self.config)
    }
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// AMQP 1.0 Session
#[derive(Debug, Clone)]
pub struct Session {
    /// Channel number
    channel: u16,
    /// Session ID
    id: String,
    /// Session state
    state: SessionState,
}

/// AMQP 1.0 Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Session is being established
    Opening,
    /// Session is open and ready
    Open,
    /// Session is being closed
    Closing,
    /// Session is closed
    Closed,
    /// Session is in error state
    Error(String),
}

impl Session {
    /// Create a new session
    pub fn new(channel: u16, connection_id: String) -> Self {
        Session {
            channel,
            id: format!("{}-session-{}", connection_id, channel),
            state: SessionState::Closed,
        }
    }

    /// Open the session
    pub async fn open(&mut self) -> AmqpResult<()> {
        if self.state != SessionState::Closed {
            return Err(AmqpError::invalid_state("Session is not in closed state"));
        }

        self.state = SessionState::Opening;
        // In a real implementation, you would send the Begin performative here
        self.state = SessionState::Open;
        Ok(())
    }

    /// Close the session
    pub async fn close(&mut self) -> AmqpResult<()> {
        if self.state != SessionState::Open {
            return Err(AmqpError::invalid_state("Session is not open"));
        }

        self.state = SessionState::Closing;
        // In a real implementation, you would send the End performative here
        self.state = SessionState::Closed;
        Ok(())
    }

    /// Get session state
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get channel number
    pub fn channel(&self) -> u16 {
        self.channel
    }
}

/// Session Builder for constructing AMQP 1.0 sessions
#[derive(Debug, Clone)]
pub struct SessionBuilder {
    channel: u16,
}

impl SessionBuilder {
    /// Create a new session builder
    pub fn new(channel: u16) -> Self {
        SessionBuilder { channel }
    }

    /// Build the session
    pub fn build(self, connection_id: String) -> Session {
        Session::new(self.channel, connection_id)
    }
} 
//! AMQP 1.0 Network Layer
//!
//! This module provides the network layer for AMQP 1.0, including TCP connections,
//! protocol negotiation, frame handling, and message transmission.
//!
//! # Overview
//!
//! The network layer handles:
//! - TCP connection establishment and management
//! - AMQP protocol negotiation
//! - Frame encoding and decoding
//! - Message transmission and reception
//! - Connection keep-alive and heartbeat
//!
//! # Examples
//!
//! ## Basic Network Connection
//!
//! ```rust
//! use dumq_amqp::network::{NetworkConnection, NetworkBuilder};
//! use tokio::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut connection = NetworkBuilder::new()
//!         .hostname("localhost")
//!         .port(5672)
//!         .timeout(Duration::from_secs(30))
//!         .build();
//!
//!     connection.connect().await?;
//!     connection.negotiate_protocol().await?;
//!
//!     // Use connection...
//!
//!     connection.disconnect().await?;
//!     Ok(())
//! }
//! ```

use crate::{AmqpError, AmqpResult, AmqpValue, AmqpSymbol};
use crate::codec::{Encoder, Decoder};
use crate::transport::{Frame, FrameHeader, FrameType, Transport, TransportBuilder};
use crate::types::AmqpMap;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::{sleep};
use uuid::Uuid;

/// Network connection state
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkState {
    /// Not connected
    Disconnected,
    /// Connecting to remote host
    Connecting,
    /// Connected but protocol not negotiated
    Connected,
    /// Protocol negotiated and ready
    Ready,
    /// Connection is closing
    Closing,
    /// Connection is closed
    Closed,
    /// Connection is in error state
    Error(String),
}

/// Network connection configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Remote hostname
    pub hostname: String,
    /// Remote port
    pub port: u16,
    /// Connection timeout
    pub timeout: Duration,
    /// Keep-alive interval
    pub keep_alive: Duration,
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

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            hostname: "localhost".to_string(),
            port: 5672,
            timeout: Duration::from_secs(30),
            keep_alive: Duration::from_secs(60),
            max_frame_size: 65536,
            channel_max: 1000,
            idle_timeout: Duration::from_secs(60),
            container_id: format!("dumq-amqp-{}", Uuid::new_v4().to_string()[..8].to_string()),
            properties: HashMap::new(),
        }
    }
}

/// Network connection for AMQP 1.0
pub struct NetworkConnection {
    /// Connection state
    state: NetworkState,
    /// Connection configuration
    config: NetworkConfig,
    /// Transport layer
    transport: Option<Transport>,
    /// Connection ID
    id: String,
    /// Next channel number
    next_channel: u16,
    /// Last activity timestamp
    last_activity: Instant,
    /// Keep-alive task handle
    keep_alive_handle: Option<tokio::task::JoinHandle<()>>,
}

impl NetworkConnection {
    /// Create a new network connection
    pub fn new(config: NetworkConfig) -> Self {
        NetworkConnection {
            state: NetworkState::Disconnected,
            config,
            transport: None,
            id: format!("conn-{}", Uuid::new_v4().to_string()[..8].to_string()),
            next_channel: 0,
            last_activity: Instant::now(),
            keep_alive_handle: None,
        }
    }

    /// Connect to the remote host
    pub async fn connect(&mut self) -> AmqpResult<()> {
        if self.state != NetworkState::Disconnected {
            return Err(AmqpError::connection("Connection already established"));
        }

        self.state = NetworkState::Connecting;

        // Create transport connection
        let transport = TransportBuilder::new()
            .hostname(self.config.hostname.clone())
            .port(self.config.port)
            .timeout(self.config.timeout)
            .connect()
            .await?;

        self.transport = Some(transport);
        self.state = NetworkState::Connected;
        self.last_activity = Instant::now();

        Ok(())
    }

    /// Negotiate AMQP protocol
    pub async fn negotiate_protocol(&mut self) -> AmqpResult<()> {
        if self.state != NetworkState::Connected {
            return Err(AmqpError::connection("Not connected"));
        }

        let transport = self.transport.as_mut()
            .ok_or_else(|| AmqpError::connection("No transport available"))?;

        // Send AMQP protocol header
        Self::send_protocol_header(transport).await?;

        // Send Open performative
        Self::send_open(transport, &self.config).await?;

        // Start keep-alive task
        self.start_keep_alive();

        self.state = NetworkState::Ready;
        self.last_activity = Instant::now();

        Ok(())
    }

    /// Send a frame
    pub async fn send_frame(&mut self, frame: Frame) -> AmqpResult<()> {
        if self.state != NetworkState::Ready {
            return Err(AmqpError::connection("Connection not ready"));
        }

        let transport = self.transport.as_mut()
            .ok_or_else(|| AmqpError::connection("No transport available"))?;

        transport.send_frame(frame).await?;
        self.last_activity = Instant::now();

        Ok(())
    }

    /// Receive a frame
    pub async fn receive_frame(&mut self) -> AmqpResult<Frame> {
        if self.state != NetworkState::Ready {
            return Err(AmqpError::connection("Connection not ready"));
        }

        let transport = self.transport.as_mut()
            .ok_or_else(|| AmqpError::connection("No transport available"))?;

        let frame = transport.receive_frame().await?;
        self.last_activity = Instant::now();

        Ok(frame)
    }

    /// Send a message
    pub async fn send_message(&mut self, channel: u16, message: &crate::message::Message) -> AmqpResult<()> {
        // Encode message
        let mut encoder = Encoder::new();
        encoder.encode_message(message)?;
        let payload = encoder.finish();

        // Create frame
        let header = FrameHeader::new(payload.len() as u32, FrameType::AMQP as u8, channel);
        let frame = Frame::new(header, payload);

        self.send_frame(frame).await
    }

    /// Receive a message
    pub async fn receive_message(&mut self) -> AmqpResult<Option<crate::message::Message>> {
        let frame = self.receive_frame().await?;
        
        if frame.header.frame_type == FrameType::AMQP as u8 {
            let mut decoder = Decoder::new(frame.payload);
            let message = decoder.decode_message()?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    /// Disconnect from the remote host
    pub async fn disconnect(&mut self) -> AmqpResult<()> {
        if self.state == NetworkState::Disconnected {
            return Ok(());
        }

        self.state = NetworkState::Closing;

        // Stop keep-alive task
        if let Some(handle) = self.keep_alive_handle.take() {
            handle.abort();
        }

        // Send Close performative if connected
        if let Some(transport) = &mut self.transport {
            if self.state == NetworkState::Ready {
                Self::send_close(transport).await?;
            }
        }

        // Close transport
        if let Some(mut transport) = self.transport.take() {
            transport.shutdown().await?;
        }

        self.state = NetworkState::Closed;

        Ok(())
    }

    /// Get connection state
    pub fn state(&self) -> &NetworkState {
        &self.state
    }

    /// Get connection ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get connection configuration
    pub fn config(&self) -> &NetworkConfig {
        &self.config
    }

    /// Get next available channel number
    pub fn next_channel(&mut self) -> u16 {
        let channel = self.next_channel;
        self.next_channel = self.next_channel.wrapping_add(1);
        channel
    }

    /// Check if connection is idle
    pub fn is_idle(&self) -> bool {
        self.last_activity.elapsed() > self.config.idle_timeout
    }

    /// Send AMQP protocol header
    async fn send_protocol_header(transport: &mut Transport) -> AmqpResult<()> {
        let header = crate::transport::constants::AMQP_HEADER;
        transport.send_raw(header).await?;
        Ok(())
    }

    /// Send Open performative
    async fn send_open(transport: &mut Transport, config: &NetworkConfig) -> AmqpResult<()> {
        let mut properties = AmqpMap::new();
        for (key, value) in &config.properties {
            properties.insert(AmqpSymbol::from(key.clone()), value.clone());
        }

        // Create Open performative
        let _open_performative = vec![
            0x00, // Descriptor: Open
            0x00, 0x00, 0x00, 0x00, // Size placeholder
            // Container ID
            0xa1, // String type
        ];

        let mut encoder = Encoder::new();
        encoder.encode_value(&AmqpValue::String(config.container_id.clone()))?;
        encoder.encode_value(&AmqpValue::String("".to_string()))?; // Hostname
        encoder.encode_value(&AmqpValue::Uint(0))?; // Max frame size
        encoder.encode_value(&AmqpValue::Ushort(config.channel_max))?; // Channel max
        encoder.encode_value(&AmqpValue::Uint(config.idle_timeout.as_millis() as u32))?; // Idle timeout
        encoder.encode_value(&AmqpValue::Map(properties))?; // Properties
        encoder.encode_value(&AmqpValue::List(vec![]))?; // Offered capabilities
        encoder.encode_value(&AmqpValue::List(vec![]))?; // Desired capabilities

        let payload = encoder.finish();
        let header = FrameHeader::new(payload.len() as u32, FrameType::AMQP as u8, 0);
        let frame = Frame::new(header, payload);

        transport.send_frame(frame).await?;
        Ok(())
    }

    /// Send Close performative
    async fn send_close(transport: &mut Transport) -> AmqpResult<()> {
        let mut encoder = Encoder::new();
        encoder.encode_value(&AmqpValue::String("".to_string()))?; // Error condition
        encoder.encode_value(&AmqpValue::String("".to_string()))?; // Error description

        let payload = encoder.finish();
        let header = FrameHeader::new(payload.len() as u32, FrameType::AMQP as u8, 0);
        let frame = Frame::new(header, payload);

        transport.send_frame(frame).await?;
        Ok(())
    }

    /// Start keep-alive task
    fn start_keep_alive(&mut self) {
        let keep_alive_interval = self.config.keep_alive;

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(keep_alive_interval);
            loop {
                interval.tick().await;
                // Send heartbeat frame
                // This is a simplified implementation
                sleep(Duration::from_millis(100)).await;
            }
        });

        self.keep_alive_handle = Some(handle);
    }
}

impl Drop for NetworkConnection {
    fn drop(&mut self) {
        if let Some(handle) = self.keep_alive_handle.take() {
            handle.abort();
        }
    }
}

/// Builder for network connections
pub struct NetworkBuilder {
    config: NetworkConfig,
}

impl NetworkBuilder {
    /// Create a new network builder
    pub fn new() -> Self {
        NetworkBuilder {
            config: NetworkConfig::default(),
        }
    }

    /// Set hostname
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.config.hostname = hostname.into();
        self
    }

    /// Set port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set keep-alive interval
    pub fn keep_alive(mut self, keep_alive: Duration) -> Self {
        self.config.keep_alive = keep_alive;
        self
    }

    /// Set maximum frame size
    pub fn max_frame_size(mut self, max_frame_size: u32) -> Self {
        self.config.max_frame_size = max_frame_size;
        self
    }

    /// Set channel maximum
    pub fn channel_max(mut self, channel_max: u16) -> Self {
        self.config.channel_max = channel_max;
        self
    }

    /// Set idle timeout
    pub fn idle_timeout(mut self, idle_timeout: Duration) -> Self {
        self.config.idle_timeout = idle_timeout;
        self
    }

    /// Set container ID
    pub fn container_id(mut self, container_id: impl Into<String>) -> Self {
        self.config.container_id = container_id.into();
        self
    }

    /// Add connection property
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self {
        self.config.properties.insert(key.into(), value);
        self
    }

    /// Build the network connection
    pub fn build(self) -> NetworkConnection {
        NetworkConnection::new(self.config)
    }
}

impl Default for NetworkBuilder {
    fn default() -> Self {
        NetworkBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.hostname, "localhost");
        assert_eq!(config.port, 5672);
        assert!(!config.container_id.is_empty());
    }

    #[test]
    fn test_network_builder() {
        let connection = NetworkBuilder::new()
            .hostname("test.example.com")
            .port(5673)
            .timeout(Duration::from_secs(60))
            .container_id("test-container")
            .build();

        assert_eq!(connection.state(), &NetworkState::Disconnected);
        assert_eq!(connection.config.hostname, "test.example.com");
        assert_eq!(connection.config.port, 5673);
        assert_eq!(connection.config.container_id, "test-container");
    }

    #[test]
    fn test_network_connection_id() {
        let connection = NetworkConnection::new(NetworkConfig::default());
        assert!(!connection.id().is_empty());
        assert!(connection.id().starts_with("conn-"));
    }
} 
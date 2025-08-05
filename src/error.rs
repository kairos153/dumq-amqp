//! AMQP 1.0 Error Handling
//!
//! This module provides comprehensive error handling for AMQP 1.0 operations.
//! It defines error types for various failure scenarios and provides convenient
//! constructors for creating specific error instances.
//!
//! # Overview
//!
//! The error system is designed to provide detailed information about what went
//! wrong during AMQP operations, making it easier to debug and handle errors
//! appropriately.
//!
//! # Error Types
//!
//! - **Connection**: Errors related to connection establishment and management
//! - **Session**: Errors related to session operations
//! - **Link**: Errors related to sender and receiver links
//! - **Transport**: Low-level transport errors
//! - **Encoding/Decoding**: Errors in AMQP value serialization/deserialization
//! - **Protocol**: AMQP protocol violations
//! - **Timeout**: Operation timeouts
//! - **IO**: Standard I/O errors
//! - **Serialization**: JSON serialization errors
//! - **InvalidState**: State machine violations
//! - **NotImplemented**: Unimplemented features
//!
//! # Examples
//!
//! ## Error Handling
//!
//! ```rust
//! use dumq_amqp::error::{AmqpError, AmqpResult};
//!
//! fn handle_amqp_operation() -> AmqpResult<()> {
//!     match some_operation() {
//!         Ok(result) => Ok(result),
//!         Err(AmqpError::Connection(msg)) => {
//!             eprintln!("Connection error: {}", msg);
//!             Err(AmqpError::connection("Failed to connect"))
//!         }
//!         Err(AmqpError::Timeout(msg)) => {
//!             eprintln!("Timeout error: {}", msg);
//!             Err(AmqpError::timeout("Operation timed out"))
//!         }
//!         Err(e) => Err(e),
//!     }
//! }
//! ```
//!
//! ## Creating Custom Errors
//!
//! ```rust
//! use dumq_amqp::error::AmqpError;
//!
//! // Create specific error types
//! let conn_error = AmqpError::connection("Failed to establish connection");
//! let timeout_error = AmqpError::timeout("Operation timed out");
//! let state_error = AmqpError::invalid_state("Connection is not open");
//! ```

use thiserror::Error;
use crate::condition::AmqpCondition;

/// AMQP 1.0 specific error types
#[derive(Error, Debug)]
pub enum AmqpError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("Link error: {0}")]
    Link(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Encoding error: {0}")]
    Encoding(String),
    
    #[error("Decoding error: {0}")]
    Decoding(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    /// AMQP protocol error with condition code
    #[error("AMQP error: {condition} - {description}")]
    AmqpProtocol {
        condition: AmqpCondition,
        description: String,
    },
}

/// Result type for AMQP operations
pub type AmqpResult<T> = Result<T, AmqpError>;

impl AmqpError {
    /// Create a connection error
    pub fn connection(msg: impl Into<String>) -> Self {
        AmqpError::Connection(msg.into())
    }
    
    /// Create a session error
    pub fn session(msg: impl Into<String>) -> Self {
        AmqpError::Session(msg.into())
    }
    
    /// Create a link error
    pub fn link(msg: impl Into<String>) -> Self {
        AmqpError::Link(msg.into())
    }
    
    /// Create a transport error
    pub fn transport(msg: impl Into<String>) -> Self {
        AmqpError::Transport(msg.into())
    }
    
    /// Create an encoding error
    pub fn encoding(msg: impl Into<String>) -> Self {
        AmqpError::Encoding(msg.into())
    }
    
    /// Create a decoding error
    pub fn decoding(msg: impl Into<String>) -> Self {
        AmqpError::Decoding(msg.into())
    }
    
    /// Create a protocol error
    pub fn protocol(msg: impl Into<String>) -> Self {
        AmqpError::Protocol(msg.into())
    }
    
    /// Create a timeout error
    pub fn timeout(msg: impl Into<String>) -> Self {
        AmqpError::Timeout(msg.into())
    }
    
    /// Create an invalid state error
    pub fn invalid_state(msg: impl Into<String>) -> Self {
        AmqpError::InvalidState(msg.into())
    }
    
    /// Create a not implemented error
    pub fn not_implemented(msg: impl Into<String>) -> Self {
        AmqpError::NotImplemented(msg.into())
    }
    
    /// Create an AMQP protocol error with condition code
    pub fn amqp_protocol(condition: AmqpCondition, description: impl Into<String>) -> Self {
        AmqpError::AmqpProtocol {
            condition,
            description: description.into(),
        }
    }
    
    /// Get the error condition if this is an AMQP protocol error
    pub fn condition(&self) -> Option<&AmqpCondition> {
        match self {
            AmqpError::AmqpProtocol { condition, .. } => Some(condition),
            _ => None,
        }
    }
    
    /// Get the error code as a string
    pub fn error_code(&self) -> &str {
        match self {
            AmqpError::Connection(_) => "connection-error",
            AmqpError::Session(_) => "session-error",
            AmqpError::Link(_) => "link-error",
            AmqpError::Transport(_) => "transport-error",
            AmqpError::Encoding(_) => "encoding-error",
            AmqpError::Decoding(_) => "decoding-error",
            AmqpError::Protocol(_) => "protocol-error",
            AmqpError::Timeout(_) => "timeout-error",
            AmqpError::Io(_) => "io-error",
            AmqpError::Serialization(_) => "serialization-error",
            AmqpError::InvalidState(_) => "invalid-state-error",
            AmqpError::NotImplemented(_) => "not-implemented-error",
            AmqpError::AmqpProtocol { condition, .. } => condition.as_str(),
        }
    }

    pub fn error_code_num(&self) -> u16 {
        match self {
            AmqpError::AmqpProtocol { condition, .. } => condition.code_num(),
            _ => 500,
        }
    }
} 
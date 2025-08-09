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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::AmqpCondition;

    #[test]
    fn test_connection_error_creation() {
        let error = AmqpError::connection("Failed to connect");
        assert!(matches!(error, AmqpError::Connection(_)));
        assert_eq!(error.error_code(), "connection-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_session_error_creation() {
        let error = AmqpError::session("Session creation failed");
        assert!(matches!(error, AmqpError::Session(_)));
        assert_eq!(error.error_code(), "session-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_link_error_creation() {
        let error = AmqpError::link("Link attachment failed");
        assert!(matches!(error, AmqpError::Link(_)));
        assert_eq!(error.error_code(), "link-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_transport_error_creation() {
        let error = AmqpError::transport("Transport layer error");
        assert!(matches!(error, AmqpError::Transport(_)));
        assert_eq!(error.error_code(), "transport-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_encoding_error_creation() {
        let error = AmqpError::encoding("Failed to encode message");
        assert!(matches!(error, AmqpError::Encoding(_)));
        assert_eq!(error.error_code(), "encoding-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_decoding_error_creation() {
        let error = AmqpError::decoding("Failed to decode message");
        assert!(matches!(error, AmqpError::Decoding(_)));
        assert_eq!(error.error_code(), "decoding-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_protocol_error_creation() {
        let error = AmqpError::protocol("Protocol violation");
        assert!(matches!(error, AmqpError::Protocol(_)));
        assert_eq!(error.error_code(), "protocol-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_timeout_error_creation() {
        let error = AmqpError::timeout("Operation timed out");
        assert!(matches!(error, AmqpError::Timeout(_)));
        assert_eq!(error.error_code(), "timeout-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_invalid_state_error_creation() {
        let error = AmqpError::invalid_state("Invalid connection state");
        assert!(matches!(error, AmqpError::InvalidState(_)));
        assert_eq!(error.error_code(), "invalid-state-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_not_implemented_error_creation() {
        let error = AmqpError::not_implemented("Feature not implemented");
        assert!(matches!(error, AmqpError::NotImplemented(_)));
        assert_eq!(error.error_code(), "not-implemented-error");
        assert_eq!(error.error_code_num(), 500);
    }

    #[test]
    fn test_amqp_protocol_error_creation() {
        let condition = AmqpCondition::AmqpErrorInternalError;
        let error = AmqpError::amqp_protocol(condition.clone(), "Internal server error");
        
        assert!(matches!(error, AmqpError::AmqpProtocol { .. }));
        assert_eq!(error.error_code(), "amqp:internal-error");
        assert_eq!(error.error_code_num(), 500);
        
        if let AmqpError::AmqpProtocol { condition: error_condition, description } = &error {
            assert_eq!(error_condition, &condition);
            assert_eq!(description, "Internal server error");
        }
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let amqp_error: AmqpError = io_error.into();
        
        assert!(matches!(amqp_error, AmqpError::Io(_)));
        assert_eq!(amqp_error.error_code(), "io-error");
        assert_eq!(amqp_error.error_code_num(), 500);
    }

    #[test]
    fn test_serialization_error_conversion() {
        let json_error = serde_json::from_str::<String>("invalid json").unwrap_err();
        let amqp_error: AmqpError = json_error.into();
        
        assert!(matches!(amqp_error, AmqpError::Serialization(_)));
        assert_eq!(amqp_error.error_code(), "serialization-error");
        assert_eq!(amqp_error.error_code_num(), 500);
    }

    #[test]
    fn test_error_condition_access() {
        let condition = AmqpCondition::AmqpErrorConnectionForced;
        let error = AmqpError::amqp_protocol(condition.clone(), "Connection forced");
        
        assert_eq!(error.condition(), Some(&condition));
        
        // Non-protocol errors should return None
        let connection_error = AmqpError::connection("Test");
        assert_eq!(connection_error.condition(), None);
    }

    #[test]
    fn test_error_display() {
        let connection_error = AmqpError::connection("Failed to connect");
        let display_string = connection_error.to_string();
        assert!(display_string.contains("Connection error: Failed to connect"));
        
        let timeout_error = AmqpError::timeout("Operation timed out");
        let display_string = timeout_error.to_string();
        assert!(display_string.contains("Timeout error: Operation timed out"));
        
        let protocol_error = AmqpError::amqp_protocol(
            AmqpCondition::AmqpErrorInternalError,
            "Internal error"
        );
        let display_string = protocol_error.to_string();
        assert!(display_string.contains("AMQP error: amqp:internal-error - Internal error"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = AmqpError::connection("Failed to connect");
        let error2 = AmqpError::connection("Failed to connect");
        let error3 = AmqpError::connection("Different error");
        
        // Note: AmqpError doesn't implement PartialEq, so we can't test equality
        // This test demonstrates the current behavior
        assert!(matches!(error1, AmqpError::Connection(_)));
        assert!(matches!(error2, AmqpError::Connection(_)));
        assert!(matches!(error3, AmqpError::Connection(_)));
    }

    #[test]
    fn test_error_code_consistency() {
        // Test that all error types return consistent error codes
        let errors = vec![
            AmqpError::connection("test"),
            AmqpError::session("test"),
            AmqpError::link("test"),
            AmqpError::transport("test"),
            AmqpError::encoding("test"),
            AmqpError::decoding("test"),
            AmqpError::protocol("test"),
            AmqpError::timeout("test"),
            AmqpError::invalid_state("test"),
            AmqpError::not_implemented("test"),
        ];
        
        for error in errors {
            assert!(!error.error_code().is_empty());
            assert_eq!(error.error_code_num(), 500);
        }
    }

    #[test]
    fn test_amqp_protocol_error_codes() {
        let conditions = vec![
            AmqpCondition::AmqpErrorConnectionForced,
            AmqpCondition::AmqpErrorInternalError,
            AmqpCondition::AmqpErrorNotImplemented,
            AmqpCondition::AmqpErrorUnauthorizedAccess,
        ];
        
        for condition in conditions {
            let error = AmqpError::amqp_protocol(condition.clone(), "Test error");
            assert_eq!(error.error_code_num(), condition.code_num());
            assert_eq!(error.error_code(), condition.as_str());
        }
    }

    #[test]
    fn test_error_message_content() {
        let test_message = "This is a test error message";
        let error = AmqpError::connection(test_message);
        
        if let AmqpError::Connection(msg) = error {
            assert_eq!(msg, test_message);
        } else {
            panic!("Expected Connection error");
        }
    }

    #[test]
    fn test_error_with_different_message_types() {
        // Test with String
        let string_msg = "String message".to_string();
        let error1 = AmqpError::connection(string_msg.clone());
        assert!(matches!(error1, AmqpError::Connection(_)));
        
        // Test with &str
        let str_msg = "&str message";
        let error2 = AmqpError::connection(str_msg);
        assert!(matches!(error2, AmqpError::Connection(_)));
        
        // Test with owned String
        let error3 = AmqpError::connection(string_msg);
        assert!(matches!(error3, AmqpError::Connection(_)));
    }

    #[test]
    fn test_error_chaining() {
        // Test that errors can be chained through Result types
        let result: AmqpResult<()> = Err(AmqpError::connection("First error"));
        
        let chained_result = result.map_err(|e| match e {
            AmqpError::Connection(msg) => AmqpError::session(format!("Chained: {}", msg)),
            _ => e,
        });
        
        assert!(chained_result.is_err());
        if let Err(AmqpError::Session(msg)) = chained_result {
            assert!(msg.contains("Chained: First error"));
        } else {
            panic!("Expected Session error");
        }
    }

    #[test]
    fn test_error_with_empty_messages() {
        // Test with empty string
        let empty_error = AmqpError::connection("");
        assert!(matches!(empty_error, AmqpError::Connection(_)));
        assert_eq!(empty_error.error_code(), "connection-error");
        
        // Test with whitespace-only string
        let whitespace_error = AmqpError::timeout("   ");
        assert!(matches!(whitespace_error, AmqpError::Timeout(_)));
        assert_eq!(whitespace_error.error_code(), "timeout-error");
    }

    #[test]
    fn test_error_with_special_characters() {
        // Test with special characters
        let special_chars = "Error with special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
        let error = AmqpError::protocol(special_chars);
        assert!(matches!(error, AmqpError::Protocol(_)));
        
        if let AmqpError::Protocol(msg) = &error {
            assert_eq!(msg, special_chars);
        }
    }

    #[test]
    fn test_error_with_unicode() {
        // Test with unicode characters
        let unicode_msg = "Error with unicode: 🚀 你好 мир";
        let error = AmqpError::encoding(unicode_msg);
        assert!(matches!(error, AmqpError::Encoding(_)));
        
        if let AmqpError::Encoding(msg) = &error {
            assert_eq!(msg, unicode_msg);
        }
    }

    #[test]
    fn test_error_with_long_messages() {
        // Test with very long message
        let long_msg = "A".repeat(1000);
        let error = AmqpError::transport(&long_msg);
        assert!(matches!(error, AmqpError::Transport(_)));
        
        if let AmqpError::Transport(msg) = &error {
            assert_eq!(msg.len(), 1000);
            assert!(msg.chars().all(|c| c == 'A'));
        }
    }

    #[test]
    fn test_error_condition_edge_cases() {
        // Test with custom condition
        let custom_condition = AmqpCondition::Custom("custom:error".to_string());
        let error = AmqpError::amqp_protocol(custom_condition.clone(), "Custom error");
        
        assert_eq!(error.condition(), Some(&custom_condition));
        assert_eq!(error.error_code(), "custom:error");
        
        // Test that non-protocol errors return None for condition
        let connection_error = AmqpError::connection("Test");
        assert_eq!(connection_error.condition(), None);
    }

    #[test]
    fn test_error_code_consistency_across_types() {
        // Test that all error types have consistent behavior
        let test_message = "test message";
        
        let errors = vec![
            AmqpError::connection(test_message),
            AmqpError::session(test_message),
            AmqpError::link(test_message),
            AmqpError::transport(test_message),
            AmqpError::encoding(test_message),
            AmqpError::decoding(test_message),
            AmqpError::protocol(test_message),
            AmqpError::timeout(test_message),
            AmqpError::invalid_state(test_message),
            AmqpError::not_implemented(test_message),
        ];
        
        for error in errors {
            // All non-protocol errors should return 500
            assert_eq!(error.error_code_num(), 500);
            
            // All errors should have non-empty error codes
            assert!(!error.error_code().is_empty());
            
            // All errors should implement Display
            let display_string = error.to_string();
            assert!(!display_string.is_empty());
            assert!(display_string.contains(test_message));
        }
    }

    #[test]
    fn test_error_from_std_io_error() {
        // Test conversion from std::io::Error
        let io_error = std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "Connection refused by peer"
        );
        
        let amqp_error: AmqpError = io_error.into();
        assert!(matches!(amqp_error, AmqpError::Io(_)));
        assert_eq!(amqp_error.error_code(), "io-error");
        assert_eq!(amqp_error.error_code_num(), 500);
    }

    #[test]
    fn test_error_from_serde_json_error() {
        // Test conversion from serde_json::Error
        let json_str = r#"{"invalid": json"#; // Missing closing brace
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        
        let amqp_error: AmqpError = json_error.into();
        assert!(matches!(amqp_error, AmqpError::Serialization(_)));
        assert_eq!(amqp_error.error_code(), "serialization-error");
        assert_eq!(amqp_error.error_code_num(), 500);
    }

    #[test]
    fn test_error_message_ownership() {
        // Test that error constructors take ownership properly
        let original_message = "Original message".to_string();
        let error = AmqpError::connection(original_message.clone());
        
        // The error should own the message
        if let AmqpError::Connection(msg) = &error {
            assert_eq!(msg, "Original message");
        }
        
        // The original message should still be available
        assert_eq!(original_message, "Original message");
    }

    #[test]
    fn test_error_with_numeric_conditions() {
        // Test AMQP protocol errors with different condition codes
        let conditions = vec![
            (AmqpCondition::AmqpErrorConnectionForced, 301),
            (AmqpCondition::AmqpErrorFramingError, 501),
            (AmqpCondition::AmqpErrorInternalError, 500),
            (AmqpCondition::AmqpErrorNotImplemented, 501),
            (AmqpCondition::AmqpErrorUnauthorizedAccess, 401),
        ];
        
        for (condition, expected_code) in conditions {
            let error = AmqpError::amqp_protocol(condition.clone(), "Test error");
            assert_eq!(error.error_code_num(), expected_code);
            assert_eq!(error.error_code(), condition.as_str());
            assert_eq!(error.condition(), Some(&condition));
        }
    }

    #[test]
    fn test_error_display_formatting() {
        // Test that error display formatting is consistent
        let test_cases = vec![
            (AmqpError::connection("Test"), "Connection error: Test"),
            (AmqpError::session("Test"), "Session error: Test"),
            (AmqpError::link("Test"), "Link error: Test"),
            (AmqpError::transport("Test"), "Transport error: Test"),
            (AmqpError::encoding("Test"), "Encoding error: Test"),
            (AmqpError::decoding("Test"), "Decoding error: Test"),
            (AmqpError::protocol("Test"), "Protocol error: Test"),
            (AmqpError::timeout("Test"), "Timeout error: Test"),
            (AmqpError::invalid_state("Test"), "Invalid state: Test"),
            (AmqpError::not_implemented("Test"), "Not implemented: Test"),
        ];
        
        for (error, expected_prefix) in test_cases {
            let display_string = error.to_string();
            assert!(display_string.starts_with(expected_prefix));
        }
    }

    #[test]
    fn test_error_result_type() {
        // Test that AmqpResult works correctly with both Ok and Err variants
        let success_result: AmqpResult<String> = Ok("success".to_string());
        let error_result: AmqpResult<String> = Err(AmqpError::connection("error"));
        
        assert!(success_result.is_ok());
        assert!(error_result.is_err());
        
        if let Ok(value) = success_result {
            assert_eq!(value, "success");
        }
        
        if let Err(AmqpError::Connection(msg)) = error_result {
            assert_eq!(msg, "error");
        }
    }
} 
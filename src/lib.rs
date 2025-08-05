//! AMQP 1.0 Protocol Implementation in Rust
//! 
//! This library provides a complete implementation of the AMQP 1.0 messaging protocol.
//! It supports both client and server roles, with async/await support.
//!
//! # Features
//!
//! - **Full AMQP 1.0 Protocol Support**: Complete implementation of the AMQP 1.0 specification
//! - **Async/Await**: Built on top of Tokio for high-performance async operations
//! - **Type Safety**: Strongly typed AMQP values and messages
//! - **Builder Pattern**: Fluent builder APIs for easy configuration
//! - **Error Handling**: Comprehensive error types with detailed error messages
//! - **Extensible**: Modular design for easy extension and customization
//!
//! # Quick Start
//!
//! ```rust
//! use dumq_amqp::prelude::*;
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
//!     // Create a session
//!     let mut session = connection.create_session().await?;
//!     session.begin().await?;
//!
//!     // Create a sender
//!     let mut sender = LinkBuilder::new()
//!         .name("my-sender")
//!         .target("my-queue")
//!         .build_sender(session.id().to_string());
//!
//!     sender.attach().await?;
//!     sender.add_credit(10);
//!
//!     // Send a message
//!     let message = Message::text("Hello, AMQP!");
//!     let delivery_id = sender.send(message).await?;
//!     println!("Message sent with delivery ID: {}", delivery_id);
//!
//!     // Clean up
//!     sender.detach().await?;
//!     session.end().await?;
//!     connection.close().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Core Concepts
//!
//! ## AMQP Values
//!
//! The library provides strongly typed AMQP values:
//!
//! ```rust
//! use dumq_amqp::types::AmqpValue;
//!
//! let values = vec![
//!     AmqpValue::String("Hello".to_string()),
//!     AmqpValue::Int(42),
//!     AmqpValue::Boolean(true),
//!     AmqpValue::Double(3.14159),
//!     AmqpValue::Uuid(uuid::Uuid::new_v4()),
//!     AmqpValue::Binary(vec![1, 2, 3, 4]),
//! ];
//! ```
//!
//! ## Messages
//!
//! Create and manipulate AMQP messages:
//!
//! ```rust
//! use dumq_amqp::message::Message;
//!
//! // Simple text message
//! let text_msg = Message::text("Hello, World!");
//!
//! // Binary message
//! let binary_msg = Message::binary(b"Binary data");
//!
//! // Message with properties
//! let complex_msg = Message::builder()
//!     .body(message::Body::Value(AmqpValue::String("Custom content".to_string())))
//!     .build()
//!     .with_message_id("msg-001")
//!     .with_subject("Test Message")
//!     .with_content_type(AmqpSymbol::from("text/plain"));
//! ```
//!
//! ## Encoding/Decoding
//!
//! The library includes efficient binary encoding and decoding:
//!
//! ```rust
//! use dumq_amqp::codec::{Encoder, Decoder};
//!
//! let value = AmqpValue::String("Hello, AMQP!".to_string());
//!
//! // Encode
//! let mut encoder = Encoder::new();
//! encoder.encode_value(&value)?;
//! let encoded = encoder.finish();
//!
//! // Decode
//! let mut decoder = Decoder::new(encoded);
//! let decoded = decoder.decode_value()?;
//! assert_eq!(value, decoded);
//! ```
//!
//! # Architecture
//!
//! The library is organized into several key modules:
//!
//! - **`connection`**: Connection management and lifecycle
//! - **`session`**: Session handling and flow control
//! - **`link`**: Sender and receiver link management
//! - **`message`**: AMQP message structures and manipulation
//! - **`types`**: AMQP value types and data structures
//! - **`codec`**: Binary encoding and decoding
//! - **`transport`**: Low-level transport layer
//! - **`error`**: Comprehensive error handling

pub mod types;
pub mod condition;
pub mod error;
pub mod connection;
pub mod session;
pub mod link;
pub mod message;
pub mod codec;
pub mod transport;
pub mod network;

pub use types::{AmqpValue, AmqpSymbol, AmqpList, AmqpMap, SenderSettleMode, ReceiverSettleMode, TerminusDurability, TerminusExpiryPolicy};
pub use condition::{AmqpCondition, AmqpErrorCondition, ConditionCategory};
pub use message::{Message, MessageBuilder, Properties, Header, Body};
pub use error::{AmqpError, AmqpResult};
pub use connection::{Connection, ConnectionBuilder};
pub use session::{Session, SessionBuilder};
pub use link::{Link, LinkBuilder, Sender, Receiver};
pub use network::{NetworkConnection, NetworkBuilder, NetworkConfig, NetworkState};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        Connection, ConnectionBuilder,
        Session, SessionBuilder,
        Link, LinkBuilder, Sender, Receiver,
        Message, MessageBuilder,
        AmqpError, AmqpResult,
        AmqpValue, AmqpSymbol, AmqpList, AmqpMap, AmqpCondition, AmqpErrorCondition, ConditionCategory,
        SenderSettleMode, ReceiverSettleMode, Properties, Header, Body,
        NetworkConnection, NetworkBuilder, NetworkConfig, NetworkState,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_value_creation() {
        let string_value = AmqpValue::String("test".to_string());
        let int_value = AmqpValue::Int(42);
        let bool_value = AmqpValue::Boolean(true);
        let double_value = AmqpValue::Double(3.14);
        let uuid_value = AmqpValue::Uuid(uuid::Uuid::new_v4());

        assert!(matches!(string_value, AmqpValue::String(_)));
        assert!(matches!(int_value, AmqpValue::Int(_)));
        assert!(matches!(bool_value, AmqpValue::Boolean(_)));
        assert!(matches!(double_value, AmqpValue::Double(_)));
        assert!(matches!(uuid_value, AmqpValue::Uuid(_)));
    }

    #[test]
    fn test_amqp_symbol_creation() {
        let symbol = AmqpSymbol::from("test-symbol");
        assert_eq!(symbol.as_str(), "test-symbol");
    }

    #[test]
    fn test_message_creation() {
        let message = Message::text("Hello, World!");
        assert_eq!(message.body_as_text(), Some("Hello, World!"));
    }

    #[test]
    fn test_message_with_properties() {
        let message = Message::builder()
            .build()
            .with_message_id("test-msg-001")
            .with_subject("Test Subject");

        assert_eq!(message.message_id_as_string(), Some("test-msg-001".to_string()));
        assert_eq!(
            message.properties.as_ref().and_then(|p| p.subject.as_ref()),
            Some(&"Test Subject".to_string())
        );
    }

    #[test]
    fn test_connection_builder() {
        let connection = ConnectionBuilder::new()
            .hostname("localhost")
            .port(5672)
            .container_id("test-container")
            .build();

        assert_eq!(connection.state(), &connection::ConnectionState::Closed);
        assert!(!connection.id().is_empty());
    }

    #[test]
    fn test_session_creation() {
        let session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.channel(), 1);
        assert_eq!(session.id(), "test-connection-session-1");
        assert_eq!(session.state(), &session::SessionState::Ended);
    }
} 
//! AMQP 1.0 Message System
//!
//! This module provides the message structures and manipulation utilities for AMQP 1.0.
//! Messages are the primary unit of data transfer in AMQP 1.0 and can contain various
//! types of content and metadata.
//!
//! # Overview
//!
//! AMQP 1.0 messages consist of several sections:
//!
//! - **Header**: Contains delivery-related information
//! - **Delivery Annotations**: Transport-level annotations
//! - **Message Annotations**: Application-level annotations
//! - **Properties**: Application-level properties
//! - **Application Properties**: Custom application data
//! - **Body**: The actual message content
//! - **Footer**: Transport-level trailing information
//!
//! # Examples
//!
//! ## Creating Simple Messages
//!
//! ```rust
//! use dumq_amqp::message::Message;
//!
//! // Text message
//! let text_msg = Message::text("Hello, World!");
//!
//! // Binary message
//! let binary_msg = Message::binary(b"Binary data");
//! ```
//!
//! ## Creating Complex Messages
//!
//! ```rust
//! use dumq_amqp::message::{Message, MessageBuilder};
//! use dumq_amqp::types::{AmqpValue, AmqpSymbol};
//!
//! let message = Message::builder()
//!     .body(message::Body::Value(AmqpValue::String("Custom content".to_string())))
//!     .build()
//!     .with_message_id("msg-001")
//!     .with_subject("Test Message")
//!     .with_content_type(AmqpSymbol::from("text/plain"));
//! ```
//!
//! ## Accessing Message Content
//!
//! ```rust
//! use dumq_amqp::message::Message;
//!
//! let message = Message::text("Hello, World!");
//!
//! // Get text content
//! if let Some(text) = message.body_as_text() {
//!     println!("Message text: {}", text);
//! }
//!
//! // Get binary content
//! if let Some(binary) = message.body_as_binary() {
//!     println!("Message binary: {:?}", binary);
//! }
//! ```

use crate::{AmqpMap, AmqpSymbol, AmqpValue, types::AmqpList};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// AMQP 1.0 Message structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Message header
    pub header: Option<Header>,
    /// Message delivery annotations
    pub delivery_annotations: Option<AmqpMap>,
    /// Message annotations
    pub message_annotations: Option<AmqpMap>,
    /// Message properties
    pub properties: Option<Properties>,
    /// Application properties
    pub application_properties: Option<AmqpMap>,
    /// Message body
    pub body: Option<Body>,
    /// Footer
    pub footer: Option<AmqpMap>,
}

/// AMQP 1.0 Message Header
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    /// Whether the message is durable
    pub durable: Option<bool>,
    /// Priority of the message
    pub priority: Option<u8>,
    /// Time to live in milliseconds
    pub ttl: Option<u32>,
    /// Whether the message should be delivered at first head
    pub first_acquirer: Option<bool>,
    /// Delivery count
    pub delivery_count: Option<u32>,
}

impl Header {
    /// Create a new empty header
    pub fn new() -> Self {
        Header {
            durable: None,
            priority: None,
            ttl: None,
            first_acquirer: None,
            delivery_count: None,
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Header::new()
    }
}

/// AMQP 1.0 Message Properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Message ID
    pub message_id: Option<AmqpValue>,
    /// User ID
    pub user_id: Option<Vec<u8>>,
    /// To address
    pub to: Option<String>,
    /// Subject
    pub subject: Option<String>,
    /// Reply to address
    pub reply_to: Option<String>,
    /// Correlation ID
    pub correlation_id: Option<AmqpValue>,
    /// Content type
    pub content_type: Option<AmqpSymbol>,
    /// Content encoding
    pub content_encoding: Option<AmqpSymbol>,
    /// Absolute expiry time
    pub absolute_expiry_time: Option<i64>,
    /// Creation time
    pub creation_time: Option<i64>,
    /// Group ID
    pub group_id: Option<String>,
    /// Group sequence
    pub group_sequence: Option<u32>,
    /// Reply to group ID
    pub reply_to_group_id: Option<String>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            message_id: None,
            user_id: None,
            to: None,
            subject: None,
            reply_to: None,
            correlation_id: None,
            content_type: None,
            content_encoding: None,
            absolute_expiry_time: None,
            creation_time: None,
            group_id: None,
            group_sequence: None,
            reply_to_group_id: None,
        }
    }
}

impl Properties {
    /// Create a new empty properties
    pub fn new() -> Self {
        Properties::default()
    }
}

/// AMQP 1.0 Message Body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Body {
    /// Data body (binary)
    Data(Vec<u8>),
    /// Amqp value body
    Value(AmqpValue),
    /// Amqp sequence body
    Sequence(AmqpList),
    /// Multiple data sections
    Multiple(Vec<Body>),
}

/// Message Builder for constructing AMQP 1.0 messages
#[derive(Debug, Clone)]
pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> Self {
        MessageBuilder {
            message: Message {
                header: None,
                delivery_annotations: None,
                message_annotations: None,
                properties: None,
                application_properties: None,
                body: None,
                footer: None,
            },
        }
    }

    /// Set the message header
    pub fn header(mut self, header: Header) -> Self {
        self.message.header = Some(header);
        self
    }

    /// Set delivery annotations
    pub fn delivery_annotations(mut self, annotations: AmqpMap) -> Self {
        self.message.delivery_annotations = Some(annotations);
        self
    }

    /// Set message annotations
    pub fn message_annotations(mut self, annotations: AmqpMap) -> Self {
        self.message.message_annotations = Some(annotations);
        self
    }

    /// Set message properties
    pub fn properties(mut self, properties: Properties) -> Self {
        self.message.properties = Some(properties);
        self
    }

    /// Set application properties
    pub fn application_properties(mut self, properties: AmqpMap) -> Self {
        self.message.application_properties = Some(properties);
        self
    }

    /// Set message body
    pub fn body(mut self, body: Body) -> Self {
        self.message.body = Some(body);
        self
    }

    /// Set footer
    pub fn footer(mut self, footer: AmqpMap) -> Self {
        self.message.footer = Some(footer);
        self
    }

    /// Build the message
    pub fn build(self) -> Message {
        self.message
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Message {
    /// Create a new empty message
    pub fn new() -> Self {
        Message {
            header: None,
            delivery_annotations: None,
            message_annotations: None,
            properties: None,
            application_properties: None,
            body: None,
            footer: None,
        }
    }

    /// Create a new message builder
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }

    /// Create a simple text message
    pub fn text(text: impl Into<String>) -> Self {
        MessageBuilder::new()
            .body(Body::Value(AmqpValue::String(text.into())))
            .build()
    }

    /// Create a simple binary message
    pub fn binary(data: impl Into<Vec<u8>>) -> Self {
        MessageBuilder::new()
            .body(Body::Data(data.into()))
            .build()
    }

    /// Get the message body as text if it's a string value
    pub fn body_as_text(&self) -> Option<&str> {
        match &self.body {
            Some(Body::Value(AmqpValue::String(s))) => Some(s),
            _ => None,
        }
    }

    /// Get the message body as binary data
    pub fn body_as_binary(&self) -> Option<&[u8]> {
        match &self.body {
            Some(Body::Data(data)) => Some(data),
            _ => None,
        }
    }

    /// Get the message ID as a string
    pub fn message_id_as_string(&self) -> Option<String> {
        match &self.properties {
            Some(props) => match &props.message_id {
                Some(AmqpValue::String(s)) => Some(s.clone()),
                Some(AmqpValue::Uuid(uuid)) => Some(uuid.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    /// Set a simple message ID (string)
    pub fn with_message_id(mut self, id: impl Into<String>) -> Self {
        if self.properties.is_none() {
            self.properties = Some(Properties {
                message_id: None,
                user_id: None,
                to: None,
                subject: None,
                reply_to: None,
                correlation_id: None,
                content_type: None,
                content_encoding: None,
                absolute_expiry_time: None,
                creation_time: None,
                group_id: None,
                group_sequence: None,
                reply_to_group_id: None,
            });
        }
        
        if let Some(props) = &mut self.properties {
            props.message_id = Some(AmqpValue::String(id.into()));
        }
        
        self
    }

    /// Set a UUID message ID
    pub fn with_uuid_message_id(mut self, id: Uuid) -> Self {
        if self.properties.is_none() {
            self.properties = Some(Properties {
                message_id: None,
                user_id: None,
                to: None,
                subject: None,
                reply_to: None,
                correlation_id: None,
                content_type: None,
                content_encoding: None,
                absolute_expiry_time: None,
                creation_time: None,
                group_id: None,
                group_sequence: None,
                reply_to_group_id: None,
            });
        }
        
        if let Some(props) = &mut self.properties {
            props.message_id = Some(AmqpValue::Uuid(id));
        }
        
        self
    }

    /// Set the subject
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        if self.properties.is_none() {
            self.properties = Some(Properties {
                message_id: None,
                user_id: None,
                to: None,
                subject: None,
                reply_to: None,
                correlation_id: None,
                content_type: None,
                content_encoding: None,
                absolute_expiry_time: None,
                creation_time: None,
                group_id: None,
                group_sequence: None,
                reply_to_group_id: None,
            });
        }
        
        if let Some(props) = &mut self.properties {
            props.subject = Some(subject.into());
        }
        
        self
    }

    /// Set the content type
    pub fn with_content_type(mut self, content_type: impl Into<AmqpSymbol>) -> Self {
        if self.properties.is_none() {
            self.properties = Some(Properties {
                message_id: None,
                user_id: None,
                to: None,
                subject: None,
                reply_to: None,
                correlation_id: None,
                content_type: None,
                content_encoding: None,
                absolute_expiry_time: None,
                creation_time: None,
                group_id: None,
                group_sequence: None,
                reply_to_group_id: None,
            });
        }
        
        if let Some(props) = &mut self.properties {
            props.content_type = Some(content_type.into());
        }
        
        self
    }
}

impl From<String> for Message {
    fn from(text: String) -> Self {
        Message::text(text)
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        Message::binary(data)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AmqpValue;
    use std::collections::HashMap;

    #[test]
    fn test_message_creation() {
        let message = Message::new();
        assert!(message.header.is_none());
        assert!(message.delivery_annotations.is_none());
        assert!(message.message_annotations.is_none());
        assert!(message.properties.is_none());
        assert!(message.application_properties.is_none());
        assert!(message.body.is_none());
        assert!(message.footer.is_none());
    }

    #[test]
    fn test_message_text_creation() {
        let message = Message::text("Hello, World!");
        
        assert!(message.body.is_some());
        if let Some(Body::Value(AmqpValue::String(text))) = &message.body {
            assert_eq!(text, "Hello, World!");
        } else {
            panic!("Expected text body");
        }
    }

    #[test]
    fn test_message_binary_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let message = Message::binary(data.clone());
        
        assert!(message.body.is_some());
        if let Some(Body::Data(binary_data)) = &message.body {
            assert_eq!(binary_data, &data);
        } else {
            panic!("Expected binary body");
        }
    }

    #[test]
    fn test_message_from_string() {
        let message: Message = "Test string".to_string().into();
        
        assert!(message.body.is_some());
        if let Some(Body::Value(AmqpValue::String(text))) = &message.body {
            assert_eq!(text, "Test string");
        } else {
            panic!("Expected text body");
        }
    }

    #[test]
    fn test_message_from_vec_u8() {
        let data = vec![10, 20, 30, 40];
        let message: Message = data.clone().into();
        
        assert!(message.body.is_some());
        if let Some(Body::Data(binary_data)) = &message.body {
            assert_eq!(binary_data, &data);
        } else {
            panic!("Expected binary body");
        }
    }

    #[test]
    fn test_message_builder() {
        let message = Message::builder()
            .header(Header::new())
            .body(Body::Value(AmqpValue::String("Builder test".to_string())))
            .build();
        
        assert!(message.header.is_some());
        assert!(message.body.is_some());
        
        if let Some(Body::Value(AmqpValue::String(text))) = &message.body {
            assert_eq!(text, "Builder test");
        } else {
            panic!("Expected text body");
        }
    }

    #[test]
    fn test_message_builder_with_properties() {
        let mut properties = Properties::new();
        properties.subject = Some("Test Subject".to_string());
        properties.message_id = Some(AmqpValue::String("msg-001".to_string()));
        
        let message = Message::builder()
            .properties(properties)
            .build();
        
        assert!(message.properties.is_some());
        if let Some(props) = &message.properties {
            assert_eq!(props.subject, Some("Test Subject".to_string()));
            assert_eq!(props.message_id, Some(AmqpValue::String("msg-001".to_string())));
        }
    }

    #[test]
    fn test_message_builder_with_annotations() {
        let mut annotations = HashMap::new();
        annotations.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
        annotations.insert(AmqpSymbol::from("key2"), AmqpValue::Int(42));
        
        let message = Message::builder()
            .message_annotations(annotations.clone())
            .delivery_annotations(annotations.clone())
            .application_properties(annotations.clone())
            .footer(annotations)
            .build();
        
        assert!(message.message_annotations.is_some());
        assert!(message.delivery_annotations.is_some());
        assert!(message.application_properties.is_some());
        assert!(message.footer.is_some());
    }

    #[test]
    fn test_message_body_as_text() {
        let message = Message::text("Test text");
        assert_eq!(message.body_as_text(), Some("Test text"));
        
        let message = Message::binary(vec![1, 2, 3]);
        assert_eq!(message.body_as_text(), None);
        
        let message = Message::new();
        assert_eq!(message.body_as_text(), None);
    }

    #[test]
    fn test_message_body_as_binary() {
        let data = vec![1, 2, 3, 4];
        let message = Message::binary(data.clone());
        assert_eq!(message.body_as_binary(), Some(data.as_slice()));
        
        let message = Message::text("Test text");
        assert_eq!(message.body_as_binary(), None);
        
        let message = Message::new();
        assert_eq!(message.body_as_binary(), None);
    }

    #[test]
    fn test_message_with_message_id() {
        let message = Message::new()
            .with_message_id("test-msg-001");
        
        assert!(message.properties.is_some());
        if let Some(props) = &message.properties {
            assert_eq!(props.message_id, Some(AmqpValue::String("test-msg-001".to_string())));
        }
    }

    #[test]
    fn test_message_with_uuid_message_id() {
        let uuid = Uuid::new_v4();
        let message = Message::new()
            .with_uuid_message_id(uuid);
        
        assert!(message.properties.is_some());
        if let Some(props) = &message.properties {
            assert_eq!(props.message_id, Some(AmqpValue::Uuid(uuid)));
        }
    }

    #[test]
    fn test_message_with_subject() {
        let message = Message::new()
            .with_subject("Test Subject");
        
        assert!(message.properties.is_some());
        if let Some(props) = &message.properties {
            assert_eq!(props.subject, Some("Test Subject".to_string()));
        }
    }

    #[test]
    fn test_message_with_content_type() {
        let content_type = AmqpSymbol::from("text/plain");
        let message = Message::new()
            .with_content_type(content_type.clone());
        
        assert!(message.properties.is_some());
        if let Some(props) = &message.properties {
            assert_eq!(props.content_type, Some(content_type));
        }
    }

    #[test]
    fn test_message_id_as_string() {
        let message = Message::new()
            .with_message_id("string-id");
        
        assert_eq!(message.message_id_as_string(), Some("string-id".to_string()));
        
        let uuid = Uuid::new_v4();
        let message = Message::new()
            .with_uuid_message_id(uuid);
        
        assert_eq!(message.message_id_as_string(), Some(uuid.to_string()));
        
        let message = Message::new();
        assert_eq!(message.message_id_as_string(), None);
    }

    #[test]
    fn test_header_creation() {
        let header = Header::new();
        assert!(header.durable.is_none());
        assert!(header.priority.is_none());
        assert!(header.ttl.is_none());
        assert!(header.first_acquirer.is_none());
        assert!(header.delivery_count.is_none());
    }

    #[test]
    fn test_header_default() {
        let header = Header::default();
        assert!(header.durable.is_none());
        assert!(header.priority.is_none());
        assert!(header.ttl.is_none());
        assert!(header.first_acquirer.is_none());
        assert!(header.delivery_count.is_none());
    }

    #[test]
    fn test_properties_creation() {
        let properties = Properties::new();
        assert!(properties.message_id.is_none());
        assert!(properties.user_id.is_none());
        assert!(properties.to.is_none());
        assert!(properties.subject.is_none());
        assert!(properties.reply_to.is_none());
        assert!(properties.correlation_id.is_none());
        assert!(properties.content_type.is_none());
        assert!(properties.content_encoding.is_none());
        assert!(properties.absolute_expiry_time.is_none());
        assert!(properties.creation_time.is_none());
        assert!(properties.group_id.is_none());
        assert!(properties.group_sequence.is_none());
        assert!(properties.reply_to_group_id.is_none());
    }

    #[test]
    fn test_properties_default() {
        let properties = Properties::default();
        assert!(properties.message_id.is_none());
        assert!(properties.user_id.is_none());
        assert!(properties.to.is_none());
        assert!(properties.subject.is_none());
        assert!(properties.reply_to.is_none());
        assert!(properties.correlation_id.is_none());
        assert!(properties.content_type.is_none());
        assert!(properties.content_encoding.is_none());
        assert!(properties.absolute_expiry_time.is_none());
        assert!(properties.creation_time.is_none());
        assert!(properties.group_id.is_none());
        assert!(properties.group_sequence.is_none());
        assert!(properties.reply_to_group_id.is_none());
    }

    #[test]
    fn test_body_variants() {
        // Data body
        let data_body = Body::Data(vec![1, 2, 3, 4]);
        assert!(matches!(data_body, Body::Data(_)));
        
        // Value body
        let value_body = Body::Value(AmqpValue::String("test".to_string()));
        assert!(matches!(value_body, Body::Value(_)));
        
        // Sequence body
        let sequence_body = Body::Sequence(vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
        ]);
        assert!(matches!(sequence_body, Body::Sequence(_)));
        
        // Multiple body
        let multiple_body = Body::Multiple(vec![
            Body::Data(vec![1, 2, 3]),
            Body::Value(AmqpValue::String("text".to_string())),
        ]);
        assert!(matches!(multiple_body, Body::Multiple(_)));
    }

    #[test]
    fn test_message_builder_default() {
        let builder = MessageBuilder::default();
        let message = builder.build();
        
        assert!(message.header.is_none());
        assert!(message.delivery_annotations.is_none());
        assert!(message.message_annotations.is_none());
        assert!(message.properties.is_none());
        assert!(message.application_properties.is_none());
        assert!(message.body.is_none());
        assert!(message.footer.is_none());
    }

    #[test]
    fn test_message_clone() {
        let original = Message::text("Original message");
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(cloned.body_as_text(), Some("Original message"));
    }

    #[test]
    fn test_message_equality() {
        let message1 = Message::text("Test message");
        let message2 = Message::text("Test message");
        let message3 = Message::text("Different message");
        
        assert_eq!(message1, message2);
        assert_ne!(message1, message3);
    }

    #[test]
    fn test_message_with_complex_body() {
        let complex_body = Body::Multiple(vec![
            Body::Data(vec![1, 2, 3]),
            Body::Value(AmqpValue::String("text part".to_string())),
            Body::Sequence(vec![
                AmqpValue::Int(100),
                AmqpValue::Boolean(true),
            ]),
        ]);
        
        let message = Message::builder()
            .body(complex_body)
            .build();
        
        assert!(message.body.is_some());
        if let Some(Body::Multiple(bodies)) = &message.body {
            assert_eq!(bodies.len(), 3);
            assert!(matches!(&bodies[0], Body::Data(_)));
            assert!(matches!(&bodies[1], Body::Value(_)));
            assert!(matches!(&bodies[2], Body::Sequence(_)));
        } else {
            panic!("Expected multiple body");
        }
    }

    #[test]
    fn test_serde_serialization() {
        let message = Message::text("Test message");
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(message, deserialized);
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#"{"body":{"Value":{"String":"test"}}}"#;
        let deserialized: Message = serde_json::from_str(json).unwrap();
        
        assert_eq!(deserialized.body_as_text(), Some("test"));
    }
} 
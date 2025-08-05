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
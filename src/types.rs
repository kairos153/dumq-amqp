//! AMQP 1.0 Type System
//!
//! This module provides the core type system for AMQP 1.0, including all value types,
//! symbols, lists, maps, and other data structures defined in the AMQP 1.0 specification.
//!
//! # Overview
//!
//! The AMQP 1.0 type system is designed to be rich and flexible, supporting a wide
//! range of data types from simple primitives to complex composite types.
//!
//! # Core Types
//!
//! ## AmqpValue
//!
//! The main enum representing all possible AMQP 1.0 value types:
//!
//! ```rust
//! use dumq_amqp::types::AmqpValue;
//!
//! // Primitive types
//! let null = AmqpValue::Null;
//! let boolean = AmqpValue::Boolean(true);
//! let integer = AmqpValue::Int(42);
//! let float = AmqpValue::Double(3.14159);
//! let string = AmqpValue::String("Hello".to_string());
//!
//! // Complex types
//! let uuid = AmqpValue::Uuid(uuid::Uuid::new_v4());
//! let binary = AmqpValue::Binary(vec![1, 2, 3, 4]);
//! let symbol = AmqpValue::Symbol(AmqpSymbol::from("my-symbol"));
//! ```
//!
//! ## AmqpSymbol
//!
//! Symbols are used for identifiers, property names, and other string-like values
//! that are frequently used and can be optimized:
//!
//! ```rust
//! use dumq_amqp::types::AmqpSymbol;
//!
//! let symbol = AmqpSymbol::from("my-symbol");
//! assert_eq!(symbol.as_str(), "my-symbol");
//! ```
//!
//! ## AmqpList and AmqpMap
//!
//! Composite types for structured data:
//!
//! ```rust
//! use dumq_amqp::types::{AmqpList, AmqpMap, AmqpValue, AmqpSymbol};
//! use std::collections::HashMap;
//!
//! // Create a list
//! let list = AmqpList::from(vec![
//!     AmqpValue::String("item1".to_string()),
//!     AmqpValue::Int(42),
//!     AmqpValue::Boolean(true),
//! ]);
//!
//! // Create a map
//! let mut map_data = HashMap::new();
//! map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
//! map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(123));
//! let map = AmqpMap::from(map_data);
//! ```

use serde::{Deserialize, Serialize};

/// AMQP Symbol type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AmqpSymbol(pub String);

impl AmqpSymbol {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for AmqpSymbol {
    fn from(s: String) -> Self {
        AmqpSymbol(s)
    }
}

impl From<&str> for AmqpSymbol {
    fn from(s: &str) -> Self {
        AmqpSymbol(s.to_string())
    }
}

impl std::fmt::Display for AmqpSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// AMQP List type
pub type AmqpList = Vec<AmqpValue>;

/// AMQP Map type
pub type AmqpMap = std::collections::HashMap<AmqpSymbol, AmqpValue>;

/// AMQP Value type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AmqpValue {
    Null,
    Boolean(bool),
    Ubyte(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal32(u32),
    Decimal64(u64),
    Decimal128(u128),
    Char(char),
    Timestamp(i64),
    Uuid(uuid::Uuid),
    Binary(Vec<u8>),
    String(String),
    Symbol(AmqpSymbol),
    List(AmqpList),
    Map(AmqpMap),
    Array(Vec<AmqpValue>),
}

/// AMQP Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmqpError {
    pub condition: crate::condition::AmqpCondition,
    pub description: Option<String>,
    pub info: Option<AmqpMap>,
}

impl AmqpError {
    pub fn new(condition: crate::condition::AmqpCondition) -> Self {
        AmqpError {
            condition,
            description: None,
            info: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_info(mut self, info: AmqpMap) -> Self {
        self.info = Some(info);
        self
    }
}

/// Sender Settle Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenderSettleMode {
    Unsettled = 0,
    Settled = 1,
    Mixed = 2,
}

/// Receiver Settle Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceiverSettleMode {
    First = 0,
    Second = 1,
}

/// Terminus Durability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminusDurability {
    None = 0,
    Configuration = 1,
    UnsettledState = 2,
}

/// Terminus Expiry Policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminusExpiryPolicy {
    SessionEnd = 0,
    ConnectionClose = 1,
    Never = 2,
}

/// Message Properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageProperties {
    pub message_id: Option<AmqpValue>,
    pub user_id: Option<Vec<u8>>,
    pub to: Option<String>,
    pub subject: Option<String>,
    pub reply_to: Option<String>,
    pub correlation_id: Option<AmqpValue>,
    pub content_type: Option<String>,
    pub content_encoding: Option<String>,
    pub absolute_expiry_time: Option<i64>,
    pub creation_time: Option<i64>,
    pub group_id: Option<String>,
    pub group_sequence: Option<u32>,
    pub reply_to_group_id: Option<String>,
}

/// Message Annotations
pub type MessageAnnotations = AmqpMap;

/// Application Properties
pub type ApplicationProperties = AmqpMap; 
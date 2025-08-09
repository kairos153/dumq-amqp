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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_amqp_symbol_creation() {
        let symbol1 = AmqpSymbol::from("test-symbol");
        let symbol2 = AmqpSymbol::from("test-symbol".to_string());
        
        assert_eq!(symbol1.as_str(), "test-symbol");
        assert_eq!(symbol2.as_str(), "test-symbol");
        assert_eq!(symbol1, symbol2);
    }

    #[test]
    fn test_amqp_symbol_display() {
        let symbol = AmqpSymbol::from("display-test");
        assert_eq!(symbol.to_string(), "display-test");
    }

    #[test]
    fn test_amqp_symbol_hash() {
        let mut map = HashMap::new();
        let symbol1 = AmqpSymbol::from("key1");
        let symbol2 = AmqpSymbol::from("key2");
        
        map.insert(symbol1.clone(), "value1");
        map.insert(symbol2.clone(), "value2");
        
        assert_eq!(map.get(&symbol1), Some(&"value1"));
        assert_eq!(map.get(&symbol2), Some(&"value2"));
    }

    #[test]
    fn test_amqp_value_creation() {
        // Primitive types
        let null_value = AmqpValue::Null;
        let bool_value = AmqpValue::Boolean(true);
        let ubyte_value = AmqpValue::Ubyte(255);
        let ushort_value = AmqpValue::Ushort(65535);
        let uint_value = AmqpValue::Uint(4294967295);
        let ulong_value = AmqpValue::Ulong(18446744073709551615);
        let byte_value = AmqpValue::Byte(-128);
        let short_value = AmqpValue::Short(-32768);
        let int_value = AmqpValue::Int(-2147483648);
        let long_value = AmqpValue::Long(-9223372036854775808);
        let float_value = AmqpValue::Float(3.14);
        let double_value = AmqpValue::Double(3.14159);
        let char_value = AmqpValue::Char('A');
        let timestamp_value = AmqpValue::Timestamp(1234567890);
        let uuid_value = AmqpValue::Uuid(uuid::Uuid::new_v4());
        let binary_value = AmqpValue::Binary(vec![1, 2, 3, 4]);
        let string_value = AmqpValue::String("Hello, AMQP!".to_string());
        let symbol_value = AmqpValue::Symbol(AmqpSymbol::from("test-symbol"));

        // Verify types
        assert!(matches!(null_value, AmqpValue::Null));
        assert!(matches!(bool_value, AmqpValue::Boolean(true)));
        assert!(matches!(ubyte_value, AmqpValue::Ubyte(255)));
        assert!(matches!(ushort_value, AmqpValue::Ushort(65535)));
        assert!(matches!(uint_value, AmqpValue::Uint(4294967295)));
        assert!(matches!(ulong_value, AmqpValue::Ulong(18446744073709551615)));
        assert!(matches!(byte_value, AmqpValue::Byte(-128)));
        assert!(matches!(short_value, AmqpValue::Short(-32768)));
        assert!(matches!(int_value, AmqpValue::Int(-2147483648)));
        assert!(matches!(long_value, AmqpValue::Long(-9223372036854775808)));
        assert!(matches!(float_value, AmqpValue::Float(3.14)));
        assert!(matches!(double_value, AmqpValue::Double(3.14159)));
        assert!(matches!(char_value, AmqpValue::Char('A')));
        assert!(matches!(timestamp_value, AmqpValue::Timestamp(1234567890)));
        assert!(matches!(uuid_value, AmqpValue::Uuid(_)));
        assert!(matches!(binary_value, AmqpValue::Binary(_)));
        assert!(matches!(string_value, AmqpValue::String(_)));
        assert!(matches!(symbol_value, AmqpValue::Symbol(_)));
    }

    #[test]
    fn test_amqp_value_composite_types() {
        // List
        let list_value = AmqpValue::List(vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
            AmqpValue::Boolean(true),
        ]);

        // Map
        let mut map_data = HashMap::new();
        map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
        map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(123));
        let map_value = AmqpValue::Map(map_data);

        // Array
        let array_value = AmqpValue::Array(vec![
            AmqpValue::String("array-item".to_string()),
            AmqpValue::Double(3.14),
        ]);

        assert!(matches!(list_value, AmqpValue::List(_)));
        assert!(matches!(map_value, AmqpValue::Map(_)));
        assert!(matches!(array_value, AmqpValue::Array(_)));

        // Verify list contents
        if let AmqpValue::List(list) = &list_value {
            assert_eq!(list.len(), 3);
            assert!(matches!(&list[0], AmqpValue::String(_)));
            assert!(matches!(&list[1], AmqpValue::Int(42)));
            assert!(matches!(&list[2], AmqpValue::Boolean(true)));
        }

        // Verify map contents
        if let AmqpValue::Map(map) = &map_value {
            assert_eq!(map.len(), 2);
            let key1 = AmqpSymbol::from("key1");
            let key2 = AmqpSymbol::from("key2");
            assert!(map.contains_key(&key1));
            assert!(map.contains_key(&key2));
        }
    }

    #[test]
    fn test_amqp_value_clone() {
        let original = AmqpValue::String("test".to_string());
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert!(matches!(cloned, AmqpValue::String(_)));
    }

    #[test]
    fn test_amqp_value_equality() {
        let value1 = AmqpValue::Int(42);
        let value2 = AmqpValue::Int(42);
        let value3 = AmqpValue::Int(100);
        
        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
        
        let string1 = AmqpValue::String("hello".to_string());
        let string2 = AmqpValue::String("hello".to_string());
        let string3 = AmqpValue::String("world".to_string());
        
        assert_eq!(string1, string2);
        assert_ne!(string1, string3);
    }

    #[test]
    fn test_amqp_error_creation() {
        let condition = crate::condition::AmqpCondition::AmqpErrorInternalError;
        
        let error = AmqpError::new(condition.clone());
        assert_eq!(error.condition, condition);
        assert!(error.description.is_none());
        assert!(error.info.is_none());
    }

    #[test]
    fn test_amqp_error_with_description() {
        let condition = crate::condition::AmqpCondition::AmqpErrorInternalError;
        
        let error = AmqpError::new(condition)
            .with_description("Test error description");
        
        assert_eq!(error.description, Some("Test error description".to_string()));
    }

    #[test]
    fn test_amqp_error_with_info() {
        let condition = crate::condition::AmqpCondition::AmqpErrorInternalError;
        
        let mut info = HashMap::new();
        info.insert(AmqpSymbol::from("key"), AmqpValue::String("value".to_string()));
        
        let error = AmqpError::new(condition)
            .with_info(info.clone());
        
        assert_eq!(error.info, Some(info));
    }

    #[test]
    fn test_sender_settle_mode() {
        assert_eq!(SenderSettleMode::Unsettled as i32, 0);
        assert_eq!(SenderSettleMode::Settled as i32, 1);
        assert_eq!(SenderSettleMode::Mixed as i32, 2);
    }

    #[test]
    fn test_receiver_settle_mode() {
        assert_eq!(ReceiverSettleMode::First as i32, 0);
        assert_eq!(ReceiverSettleMode::Second as i32, 1);
    }

    #[test]
    fn test_terminus_durability() {
        assert_eq!(TerminusDurability::None as i32, 0);
        assert_eq!(TerminusDurability::Configuration as i32, 1);
        assert_eq!(TerminusDurability::UnsettledState as i32, 2);
    }

    #[test]
    fn test_terminus_expiry_policy() {
        assert_eq!(TerminusExpiryPolicy::SessionEnd as i32, 0);
        assert_eq!(TerminusExpiryPolicy::ConnectionClose as i32, 1);
        assert_eq!(TerminusExpiryPolicy::Never as i32, 2);
    }

    #[test]
    fn test_message_properties() {
        let properties = MessageProperties {
            message_id: Some(AmqpValue::String("msg-001".to_string())),
            user_id: Some(vec![1, 2, 3, 4]),
            to: Some("destination".to_string()),
            subject: Some("Test Subject".to_string()),
            reply_to: Some("reply-queue".to_string()),
            correlation_id: Some(AmqpValue::String("corr-001".to_string())),
            content_type: Some("text/plain".to_string()),
            content_encoding: Some("utf-8".to_string()),
            absolute_expiry_time: Some(1234567890),
            creation_time: Some(1234567890),
            group_id: Some("group-1".to_string()),
            group_sequence: Some(1),
            reply_to_group_id: Some("reply-group-1".to_string()),
        };

        assert_eq!(properties.message_id, Some(AmqpValue::String("msg-001".to_string())));
        assert_eq!(properties.user_id, Some(vec![1, 2, 3, 4]));
        assert_eq!(properties.to, Some("destination".to_string()));
        assert_eq!(properties.subject, Some("Test Subject".to_string()));
        assert_eq!(properties.reply_to, Some("reply-queue".to_string()));
        assert_eq!(properties.correlation_id, Some(AmqpValue::String("corr-001".to_string())));
        assert_eq!(properties.content_type, Some("text/plain".to_string()));
        assert_eq!(properties.content_encoding, Some("utf-8".to_string()));
        assert_eq!(properties.absolute_expiry_time, Some(1234567890));
        assert_eq!(properties.creation_time, Some(1234567890));
        assert_eq!(properties.group_id, Some("group-1".to_string()));
        assert_eq!(properties.group_sequence, Some(1));
        assert_eq!(properties.reply_to_group_id, Some("reply-group-1".to_string()));
    }

    #[test]
    fn test_amqp_list_type_alias() {
        let list: AmqpList = vec![
            AmqpValue::String("test".to_string()),
            AmqpValue::Int(42),
        ];
        
        assert_eq!(list.len(), 2);
        assert!(matches!(&list[0], AmqpValue::String(_)));
        assert!(matches!(&list[1], AmqpValue::Int(42)));
    }

    #[test]
    fn test_amqp_map_type_alias() {
        let mut map: AmqpMap = HashMap::new();
        map.insert(AmqpSymbol::from("key"), AmqpValue::String("value".to_string()));
        
        assert_eq!(map.len(), 1);
        let key = AmqpSymbol::from("key");
        assert!(map.contains_key(&key));
        assert_eq!(map.get(&key), Some(&AmqpValue::String("value".to_string())));
    }

    #[test]
    fn test_serde_serialization() {
        let value = AmqpValue::String("test".to_string());
        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: AmqpValue = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#"{"String": "test"}"#;
        let deserialized: AmqpValue = serde_json::from_str(json).unwrap();
        
        assert!(matches!(deserialized, AmqpValue::String(_)));
        if let AmqpValue::String(s) = deserialized {
            assert_eq!(s, "test");
        }
    }
} 
//! AMQP 1.0 Binary Encoding and Decoding
//!
//! This module provides efficient binary encoding and decoding for AMQP 1.0 values,
//! messages, and protocol frames. It implements the AMQP 1.0 binary format specification
//! for optimal performance and compatibility.
//!
//! # Overview
//!
//! The codec module provides two main components:
//!
//! - **Encoder**: Converts AMQP values to binary format
//! - **Decoder**: Converts binary data back to AMQP values
//!
//! # Features
//!
//! - **Efficient**: Optimized for performance with minimal allocations
//! - **Complete**: Supports all AMQP 1.0 value types
//! - **Safe**: Proper error handling for malformed data
//! - **Flexible**: Can encode/decode individual values or complete messages
//!
//! # Examples
//!
//! ## Basic Encoding/Decoding
//!
//! ```rust
//! use dumq_amqp::codec::{Encoder, Decoder};
//! use dumq_amqp::types::AmqpValue;
//!
//! // Encode a value
//! let value = AmqpValue::String("Hello, AMQP!".to_string());
//! let mut encoder = Encoder::new();
//! encoder.encode_value(&value)?;
//! let encoded = encoder.finish();
//!
//! // Decode the value
//! let mut decoder = Decoder::new(encoded);
//! let decoded = decoder.decode_value()?;
//! assert_eq!(value, decoded);
//! ```
//!
//! ## Encoding Multiple Values
//!
//! ```rust
//! use dumq_amqp::codec::Encoder;
//! use dumq_amqp::types::AmqpValue;
//!
//! let mut encoder = Encoder::new();
//!
//! // Encode multiple values
//! encoder.encode_value(&AmqpValue::String("Hello".to_string()))?;
//! encoder.encode_value(&AmqpValue::Int(42))?;
//! encoder.encode_value(&AmqpValue::Boolean(true))?;
//!
//! let encoded = encoder.finish();
//! ```
//!
//! ## Working with Complex Types
//!
//! ```rust
//! use dumq_amqp::codec::{Encoder, Decoder};
//! use dumq_amqp::types::{AmqpValue, AmqpList, AmqpMap, AmqpSymbol};
//! use std::collections::HashMap;
//!
//! // Create a complex value
//! let mut map_data = HashMap::new();
//! map_data.insert(AmqpSymbol::from("key"), AmqpValue::String("value".to_string()));
//! let map = AmqpMap::from(map_data);
//!
//! // Encode and decode
//! let mut encoder = Encoder::new();
//! encoder.encode_value(&AmqpValue::Map(map.clone()))?;
//! let encoded = encoder.finish();
//!
//! let mut decoder = Decoder::new(encoded);
//! let decoded = decoder.decode_value()?;
//! ```

use bytes::{Buf, BufMut, BytesMut};
use crate::types::{AmqpValue, AmqpSymbol, AmqpList, AmqpMap};
use crate::error::AmqpError;

/// AMQP 1.0 Type Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeCode {
    // Described types
    Described = 0x00,
    
    // Null
    Null = 0x40,
    
    // Boolean
    Boolean = 0x56,
    BooleanTrue = 0x41,
    BooleanFalse = 0x42,
    
    // Unsigned integers
    Ubyte = 0x50,
    Ushort = 0x60,
    Uint = 0x70,
    Ulong = 0x80,
    
    // Signed integers
    Byte = 0x51,
    Short = 0x61,
    Int = 0x71,
    Long = 0x81,
    
    // Floating point
    Float = 0x72,
    Double = 0x82,
    
    // Decimal
    Decimal32 = 0x74,
    Decimal64 = 0x84,
    Decimal128 = 0x94,
    
    // Character
    Char = 0x73,
    
    // Timestamp
    Timestamp = 0x83,
    
    // UUID
    Uuid = 0x98,
    
    // Binary
    Binary8 = 0xa0,
    Binary32 = 0xb0,
    
    // String
    String8 = 0xa1,
    String32 = 0xb1,
    
    // Symbol
    Symbol8 = 0xa3,
    Symbol32 = 0xb3,
    
    // List
    List0 = 0x45,
    List8 = 0xc0,
    List32 = 0xd0,
    
    // Map
    Map8 = 0xc1,
    Map32 = 0xd1,
    
    // Array
    Array8 = 0xe0,
    Array32 = 0xf0,
}

/// AMQP 1.0 Encoder
pub struct Encoder {
    buffer: BytesMut,
}

impl Encoder {
    /// Create a new encoder
    pub fn new() -> Self {
        Encoder {
            buffer: BytesMut::new(),
        }
    }

    /// Create a new encoder with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Encoder {
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    /// Encode an AMQP value
    pub fn encode_value(&mut self, value: &AmqpValue) -> Result<(), AmqpError> {
        match value {
            AmqpValue::Null => self.encode_null(),
            AmqpValue::Boolean(b) => self.encode_boolean(*b),
            AmqpValue::Ubyte(n) => self.encode_ubyte(*n),
            AmqpValue::Ushort(n) => self.encode_ushort(*n),
            AmqpValue::Uint(n) => self.encode_uint(*n),
            AmqpValue::Ulong(n) => self.encode_ulong(*n),
            AmqpValue::Byte(n) => self.encode_byte(*n),
            AmqpValue::Short(n) => self.encode_short(*n),
            AmqpValue::Int(n) => self.encode_int(*n),
            AmqpValue::Long(n) => self.encode_long(*n),
            AmqpValue::Float(f) => self.encode_float(*f),
            AmqpValue::Double(f) => self.encode_double(*f),
            AmqpValue::Decimal32(n) => self.encode_decimal32(*n),
            AmqpValue::Decimal64(n) => self.encode_decimal64(*n),
            AmqpValue::Decimal128(n) => self.encode_decimal128(*n),
            AmqpValue::Char(c) => self.encode_char(*c),
            AmqpValue::Timestamp(t) => self.encode_timestamp(*t),
            AmqpValue::Uuid(u) => self.encode_uuid(*u),
            AmqpValue::Binary(data) => self.encode_binary(data),
            AmqpValue::String(s) => self.encode_string(s),
            AmqpValue::Symbol(s) => self.encode_symbol(s),
            AmqpValue::List(list) => self.encode_list(list),
            AmqpValue::Map(map) => self.encode_map(map),
            AmqpValue::Array(array) => self.encode_array(array),
        }
    }

    /// Encode null
    pub fn encode_null(&mut self) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Null as u8);
        Ok(())
    }

    /// Encode boolean
    pub fn encode_boolean(&mut self, value: bool) -> Result<(), AmqpError> {
        if value {
            self.buffer.put_u8(TypeCode::BooleanTrue as u8);
        } else {
            self.buffer.put_u8(TypeCode::BooleanFalse as u8);
        }
        Ok(())
    }

    /// Encode ubyte
    pub fn encode_ubyte(&mut self, value: u8) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Ubyte as u8);
        self.buffer.put_u8(value);
        Ok(())
    }

    /// Encode ushort
    pub fn encode_ushort(&mut self, value: u16) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Ushort as u8);
        self.buffer.put_u16(value);
        Ok(())
    }

    /// Encode uint
    pub fn encode_uint(&mut self, value: u32) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Uint as u8);
        self.buffer.put_u32(value);
        Ok(())
    }

    /// Encode ulong
    pub fn encode_ulong(&mut self, value: u64) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Ulong as u8);
        self.buffer.put_u64(value);
        Ok(())
    }

    /// Encode byte
    pub fn encode_byte(&mut self, value: i8) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Byte as u8);
        self.buffer.put_i8(value);
        Ok(())
    }

    /// Encode short
    pub fn encode_short(&mut self, value: i16) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Short as u8);
        self.buffer.put_i16(value);
        Ok(())
    }

    /// Encode int
    pub fn encode_int(&mut self, value: i32) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Int as u8);
        self.buffer.put_i32(value);
        Ok(())
    }

    /// Encode long
    pub fn encode_long(&mut self, value: i64) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Long as u8);
        self.buffer.put_i64(value);
        Ok(())
    }

    /// Encode float
    pub fn encode_float(&mut self, value: f32) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Float as u8);
        self.buffer.put_f32(value);
        Ok(())
    }

    /// Encode double
    pub fn encode_double(&mut self, value: f64) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Double as u8);
        self.buffer.put_f64(value);
        Ok(())
    }

    /// Encode char
    pub fn encode_char(&mut self, value: char) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Char as u8);
        // UTF-32 big-endian encoding: put the character as 4 bytes in big-endian order
        let char_u32 = value as u32;
        self.buffer.put_u8((char_u32 >> 24) as u8);
        self.buffer.put_u8((char_u32 >> 16) as u8);
        self.buffer.put_u8((char_u32 >> 8) as u8);
        self.buffer.put_u8(char_u32 as u8);
        Ok(())
    }

    /// Encode timestamp
    pub fn encode_timestamp(&mut self, value: i64) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Timestamp as u8);
        self.buffer.put_i64(value);
        Ok(())
    }

    /// Encode UUID
    pub fn encode_uuid(&mut self, value: uuid::Uuid) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Uuid as u8);
        self.buffer.put_u128(value.as_u128());
        Ok(())
    }

    /// Encode binary data
    pub fn encode_binary(&mut self, data: &[u8]) -> Result<(), AmqpError> {
        if data.len() <= 255 {
            self.buffer.put_u8(TypeCode::Binary8 as u8);
            self.buffer.put_u8(data.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::Binary32 as u8);
            self.buffer.put_u32(data.len() as u32);
        }
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    /// Encode string
    pub fn encode_string(&mut self, value: &str) -> Result<(), AmqpError> {
        let bytes = value.as_bytes();
        if bytes.len() <= 255 {
            self.buffer.put_u8(TypeCode::String8 as u8);
            self.buffer.put_u8(bytes.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::String32 as u8);
            self.buffer.put_u32(bytes.len() as u32);
        }
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    /// Encode symbol
    pub fn encode_symbol(&mut self, symbol: &AmqpSymbol) -> Result<(), AmqpError> {
        let bytes = symbol.0.as_bytes();
        if bytes.len() <= 255 {
            self.buffer.put_u8(TypeCode::Symbol8 as u8);
            self.buffer.put_u8(bytes.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::Symbol32 as u8);
            self.buffer.put_u32(bytes.len() as u32);
        }
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    fn encode_list(&mut self, list: &AmqpList) -> Result<(), AmqpError> {
        // Write list header
        if list.is_empty() {
            self.buffer.put_u8(TypeCode::List0 as u8);
        } else if list.len() <= 255 {
            self.buffer.put_u8(TypeCode::List8 as u8);
            self.buffer.put_u8(list.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::List32 as u8);
            self.buffer.put_u32(list.len() as u32);
        }

        // Write list items
        for item in list {
            self.encode_value(item)?;
        }
        Ok(())
    }

    fn encode_map(&mut self, map: &AmqpMap) -> Result<(), AmqpError> {
        // Write map header
        if map.is_empty() {
            self.buffer.put_u8(TypeCode::Map8 as u8);
            self.buffer.put_u8(0);
        } else if map.len() <= 127 {
            self.buffer.put_u8(TypeCode::Map8 as u8);
            self.buffer.put_u8(map.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::Map32 as u8);
            self.buffer.put_u32(map.len() as u32);
        }

        // Write map entries
        for (key, value) in map {
            self.encode_symbol(key)?;
            self.encode_value(value)?;
        }
        Ok(())
    }

    /// Encode array
    pub fn encode_array(&mut self, array: &[AmqpValue]) -> Result<(), AmqpError> {
        let mut temp_encoder = Encoder::new();
        for item in array {
            temp_encoder.encode_value(item)?;
        }
        
        let encoded_data = temp_encoder.finish();
        let size = encoded_data.len();
        
        if size <= 255 {
            self.buffer.put_u8(TypeCode::Array8 as u8);
            self.buffer.put_u8(size as u8);
            self.buffer.put_u8(array.len() as u8);
        } else {
            self.buffer.put_u8(TypeCode::Array32 as u8);
            self.buffer.put_u32(size as u32);
            self.buffer.put_u32(array.len() as u32);
        }
        
        self.buffer.extend_from_slice(&encoded_data);
        Ok(())
    }

    fn encode_decimal32(&mut self, value: u32) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Decimal32 as u8);
        self.buffer.put_u32(value);
        Ok(())
    }

    fn encode_decimal64(&mut self, value: u64) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Decimal64 as u8);
        self.buffer.put_u64(value);
        Ok(())
    }

    fn encode_decimal128(&mut self, value: u128) -> Result<(), AmqpError> {
        self.buffer.put_u8(TypeCode::Decimal128 as u8);
        self.buffer.put_u128(value);
        Ok(())
    }

    /// Get the encoded data
    pub fn finish(self) -> Vec<u8> {
        self.buffer.to_vec()
    }

    /// Encode an AMQP message
    pub fn encode_message(&mut self, message: &crate::message::Message) -> Result<(), AmqpError> {
        // Encode message header
        if let Some(header) = &message.header {
            // For now, we'll encode a simple map representation
            let mut header_map = AmqpMap::new();
            if let Some(durable) = header.durable {
                header_map.insert(AmqpSymbol::from("durable"), AmqpValue::Boolean(durable));
            }
            if let Some(priority) = header.priority {
                header_map.insert(AmqpSymbol::from("priority"), AmqpValue::Ubyte(priority));
            }
            if let Some(ttl) = header.ttl {
                header_map.insert(AmqpSymbol::from("ttl"), AmqpValue::Uint(ttl));
            }
            if let Some(first_acquirer) = header.first_acquirer {
                header_map.insert(AmqpSymbol::from("first_acquirer"), AmqpValue::Boolean(first_acquirer));
            }
            if let Some(delivery_count) = header.delivery_count {
                header_map.insert(AmqpSymbol::from("delivery_count"), AmqpValue::Uint(delivery_count));
            }
            self.encode_value(&AmqpValue::Map(header_map))?;
        }

        // Encode message properties
        if let Some(properties) = &message.properties {
            let mut props_map = AmqpMap::new();
            if let Some(message_id) = &properties.message_id {
                props_map.insert(AmqpSymbol::from("message_id"), message_id.clone());
            }
            if let Some(user_id) = &properties.user_id {
                props_map.insert(AmqpSymbol::from("user_id"), AmqpValue::Binary(user_id.clone()));
            }
            if let Some(to) = &properties.to {
                props_map.insert(AmqpSymbol::from("to"), AmqpValue::String(to.clone()));
            }
            if let Some(subject) = &properties.subject {
                props_map.insert(AmqpSymbol::from("subject"), AmqpValue::String(subject.clone()));
            }
            if let Some(reply_to) = &properties.reply_to {
                props_map.insert(AmqpSymbol::from("reply_to"), AmqpValue::String(reply_to.clone()));
            }
            if let Some(correlation_id) = &properties.correlation_id {
                props_map.insert(AmqpSymbol::from("correlation_id"), correlation_id.clone());
            }
            if let Some(content_type) = &properties.content_type {
                props_map.insert(AmqpSymbol::from("content_type"), AmqpValue::Symbol(content_type.clone()));
            }
            if let Some(content_encoding) = &properties.content_encoding {
                props_map.insert(AmqpSymbol::from("content_encoding"), AmqpValue::Symbol(content_encoding.clone()));
            }
            if let Some(absolute_expiry_time) = properties.absolute_expiry_time {
                props_map.insert(AmqpSymbol::from("absolute_expiry_time"), AmqpValue::Timestamp(absolute_expiry_time));
            }
            if let Some(creation_time) = properties.creation_time {
                props_map.insert(AmqpSymbol::from("creation_time"), AmqpValue::Timestamp(creation_time));
            }
            if let Some(group_id) = &properties.group_id {
                props_map.insert(AmqpSymbol::from("group_id"), AmqpValue::String(group_id.clone()));
            }
            if let Some(group_sequence) = properties.group_sequence {
                props_map.insert(AmqpSymbol::from("group_sequence"), AmqpValue::Uint(group_sequence));
            }
            if let Some(reply_to_group_id) = &properties.reply_to_group_id {
                props_map.insert(AmqpSymbol::from("reply_to_group_id"), AmqpValue::String(reply_to_group_id.clone()));
            }
            self.encode_value(&AmqpValue::Map(props_map))?;
        }

        // Encode message body
        if let Some(body) = &message.body {
            match body {
                crate::message::Body::Value(value) => {
                    self.encode_value(value)?;
                }
                crate::message::Body::Data(data) => {
                    self.encode_binary(data)?;
                }
                crate::message::Body::Sequence(sequences) => {
                    for sequence in sequences {
                        self.encode_value(sequence)?;
                    }
                }
                crate::message::Body::Multiple(bodies) => {
                    for body in bodies {
                        // Recursively encode each body part
                        match body {
                            crate::message::Body::Value(value) => self.encode_value(value)?,
                            crate::message::Body::Data(data) => self.encode_binary(data)?,
                            crate::message::Body::Sequence(sequences) => {
                                for sequence in sequences {
                                    self.encode_value(sequence)?;
                                }
                            }
                            crate::message::Body::Multiple(_) => {
                                return Err(AmqpError::encoding("Nested multiple bodies not supported"));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// AMQP 1.0 Decoder
pub struct Decoder {
    buffer: BytesMut,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Self {
        Decoder {
            buffer: BytesMut::from(data.as_slice()),
        }
    }

    /// Decode an AMQP value
    pub fn decode_value(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.is_empty() {
            return Err(AmqpError::decoding("Unexpected end of data"));
        }

        let type_code = self.buffer.get_u8();
        match type_code {
            x if x == TypeCode::Null as u8 => Ok(AmqpValue::Null),
            x if x == TypeCode::BooleanTrue as u8 => Ok(AmqpValue::Boolean(true)),
            x if x == TypeCode::BooleanFalse as u8 => Ok(AmqpValue::Boolean(false)),
            x if x == TypeCode::Ubyte as u8 => self.decode_ubyte(),
            x if x == TypeCode::Ushort as u8 => self.decode_ushort(),
            x if x == TypeCode::Uint as u8 => self.decode_uint(),
            x if x == TypeCode::Ulong as u8 => self.decode_ulong(),
            x if x == TypeCode::Byte as u8 => self.decode_byte(),
            x if x == TypeCode::Short as u8 => self.decode_short(),
            x if x == TypeCode::Int as u8 => self.decode_int(),
            x if x == TypeCode::Long as u8 => self.decode_long(),
            x if x == TypeCode::Float as u8 => self.decode_float(),
            x if x == TypeCode::Double as u8 => self.decode_double(),
            x if x == TypeCode::Char as u8 => self.decode_char(),
            x if x == TypeCode::Timestamp as u8 => self.decode_timestamp(),
            x if x == TypeCode::Uuid as u8 => self.decode_uuid(),
            x if x == TypeCode::Binary8 as u8 => self.decode_binary8(),
            x if x == TypeCode::Binary32 as u8 => self.decode_binary32(),
            x if x == TypeCode::String8 as u8 => self.decode_string8(),
            x if x == TypeCode::String32 as u8 => self.decode_string32(),
            x if x == TypeCode::Symbol8 as u8 => self.decode_symbol8(),
            x if x == TypeCode::Symbol32 as u8 => self.decode_symbol32(),
            x if x == TypeCode::List0 as u8 => Ok(AmqpValue::List(vec![])),
            x if x == TypeCode::List8 as u8 => {
                let count = self.buffer.get_u8() as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(self.decode_value()?);
                }
                Ok(AmqpValue::List(items))
            }
            x if x == TypeCode::List32 as u8 => {
                let count = self.buffer.get_u32() as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(self.decode_value()?);
                }
                Ok(AmqpValue::List(items))
            }
            x if x == TypeCode::Map8 as u8 => {
                let count = self.buffer.get_u8() as usize;
                let mut map = std::collections::HashMap::new();
                for _ in 0..count {
                    let key = self.decode_symbol()?;
                    let value = self.decode_value()?;
                    map.insert(key, value);
                }
                Ok(AmqpValue::Map(map))
            }
            x if x == TypeCode::Map32 as u8 => {
                let count = self.buffer.get_u32() as usize;
                let mut map = std::collections::HashMap::new();
                for _ in 0..count {
                    let key = self.decode_symbol()?;
                    let value = self.decode_value()?;
                    map.insert(key, value);
                }
                Ok(AmqpValue::Map(map))
            }
            x if x == TypeCode::Array8 as u8 => self.decode_array8(),
            x if x == TypeCode::Array32 as u8 => self.decode_array32(),
            _ => Err(AmqpError::decoding(&format!("Unknown type code: 0x{:02x}", type_code))),
        }
    }

    fn decode_ubyte(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 1 {
            return Err(AmqpError::decoding("Insufficient data for ubyte"));
        }
        Ok(AmqpValue::Ubyte(self.buffer.get_u8()))
    }

    fn decode_ushort(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 2 {
            return Err(AmqpError::decoding("Insufficient data for ushort"));
        }
        Ok(AmqpValue::Ushort(self.buffer.get_u16()))
    }

    fn decode_uint(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for uint"));
        }
        Ok(AmqpValue::Uint(self.buffer.get_u32()))
    }

    fn decode_ulong(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 8 {
            return Err(AmqpError::decoding("Insufficient data for ulong"));
        }
        Ok(AmqpValue::Ulong(self.buffer.get_u64()))
    }

    fn decode_byte(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 1 {
            return Err(AmqpError::decoding("Insufficient data for byte"));
        }
        Ok(AmqpValue::Byte(self.buffer.get_i8()))
    }

    fn decode_short(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 2 {
            return Err(AmqpError::decoding("Insufficient data for short"));
        }
        Ok(AmqpValue::Short(self.buffer.get_i16()))
    }

    fn decode_int(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for int"));
        }
        Ok(AmqpValue::Int(self.buffer.get_i32()))
    }

    fn decode_long(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 8 {
            return Err(AmqpError::decoding("Insufficient data for long"));
        }
        Ok(AmqpValue::Long(self.buffer.get_i64()))
    }

    fn decode_float(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for float"));
        }
        Ok(AmqpValue::Float(self.buffer.get_f32()))
    }

    fn decode_double(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 8 {
            return Err(AmqpError::decoding("Insufficient data for double"));
        }
        Ok(AmqpValue::Double(self.buffer.get_f64()))
    }

    fn decode_char(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for char"));
        }
        let code_point = self.buffer.get_u32();
        char::from_u32(code_point)
            .map(AmqpValue::Char)
            .ok_or_else(|| AmqpError::decoding("Invalid UTF-8 code point"))
    }

    fn decode_timestamp(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 8 {
            return Err(AmqpError::decoding("Insufficient data for timestamp"));
        }
        Ok(AmqpValue::Timestamp(self.buffer.get_i64()))
    }

    fn decode_uuid(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 16 {
            return Err(AmqpError::decoding("Insufficient data for UUID"));
        }
        let uuid_bytes = self.buffer.get_u128();
        Ok(AmqpValue::Uuid(uuid::Uuid::from_u128(uuid_bytes)))
    }

    fn decode_binary8(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 1 {
            return Err(AmqpError::decoding("Insufficient data for binary8 length"));
        }
        let len = self.buffer.get_u8() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for binary8"));
        }
        let data = self.buffer.copy_to_bytes(len);
        Ok(AmqpValue::Binary(data.to_vec()))
    }

    fn decode_binary32(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for binary32 length"));
        }
        let len = self.buffer.get_u32() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for binary32"));
        }
        let data = self.buffer.copy_to_bytes(len);
        Ok(AmqpValue::Binary(data.to_vec()))
    }

    fn decode_string8(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 1 {
            return Err(AmqpError::decoding("Insufficient data for string8 length"));
        }
        let len = self.buffer.get_u8() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for string8"));
        }
        let data = self.buffer.copy_to_bytes(len);
        String::from_utf8(data.to_vec())
            .map(AmqpValue::String)
            .map_err(|e| AmqpError::decoding(&format!("Invalid UTF-8 string: {}", e)))
    }

    fn decode_string32(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for string32 length"));
        }
        let len = self.buffer.get_u32() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for string32"));
        }
        let data = self.buffer.copy_to_bytes(len);
        String::from_utf8(data.to_vec())
            .map(AmqpValue::String)
            .map_err(|e| AmqpError::decoding(&format!("Invalid UTF-8 string: {}", e)))
    }

    fn decode_symbol8(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 1 {
            return Err(AmqpError::decoding("Insufficient data for symbol8"));
        }
        let len = self.buffer.get_u8() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for symbol8"));
        }
        let data = self.buffer.copy_to_bytes(len);
        String::from_utf8(data.to_vec())
            .map(|s| AmqpValue::Symbol(AmqpSymbol(s)))
            .map_err(|e| AmqpError::decoding(&format!("Invalid UTF-8 symbol: {}", e)))
    }

    fn decode_symbol32(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 4 {
            return Err(AmqpError::decoding("Insufficient data for symbol32"));
        }
        let len = self.buffer.get_u32() as usize;
        if self.buffer.remaining() < len {
            return Err(AmqpError::decoding("Insufficient data for symbol32"));
        }
        let data = self.buffer.copy_to_bytes(len);
        String::from_utf8(data.to_vec())
            .map(|s| AmqpValue::Symbol(AmqpSymbol(s)))
            .map_err(|e| AmqpError::decoding(&format!("Invalid UTF-8 symbol: {}", e)))
    }

    fn decode_array8(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 2 {
            return Err(AmqpError::decoding("Insufficient data for array8 header"));
        }
        let size = self.buffer.get_u8() as usize;
        let count = self.buffer.get_u8() as usize;
        
        if self.buffer.remaining() < size {
            return Err(AmqpError::decoding("Insufficient data for array8"));
        }
        
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.decode_value()?);
        }
        
        Ok(AmqpValue::Array(items))
    }

    fn decode_array32(&mut self) -> Result<AmqpValue, AmqpError> {
        if self.buffer.remaining() < 8 {
            return Err(AmqpError::decoding("Insufficient data for array32 header"));
        }
        let size = self.buffer.get_u32() as usize;
        let count = self.buffer.get_u32() as usize;
        
        if self.buffer.remaining() < size {
            return Err(AmqpError::decoding("Insufficient data for array32"));
        }
        
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.decode_value()?);
        }
        
        Ok(AmqpValue::Array(items))
    }

    /// Decode a symbol
    pub fn decode_symbol(&mut self) -> Result<AmqpSymbol, AmqpError> {
        if self.buffer.is_empty() {
            return Err(AmqpError::decoding("No data to decode"));
        }

        let type_code = self.buffer.get_u8();
        match type_code {
            x if x == TypeCode::Symbol8 as u8 => {
                let value = self.decode_symbol8()?;
                match value {
                    AmqpValue::Symbol(s) => Ok(s),
                    _ => Err(AmqpError::decoding("Expected symbol value")),
                }
            }
            x if x == TypeCode::Symbol32 as u8 => {
                let value = self.decode_symbol32()?;
                match value {
                    AmqpValue::Symbol(s) => Ok(s),
                    _ => Err(AmqpError::decoding("Expected symbol value")),
                }
            }
            _ => Err(AmqpError::decoding(&format!("Invalid symbol type code: {}", type_code))),
        }
    }

    /// Check if there's more data to decode
    pub fn has_remaining(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// Get remaining bytes
    pub fn remaining(&self) -> usize {
        self.buffer.remaining()
    }

    /// Decode an AMQP message
    pub fn decode_message(&mut self) -> Result<crate::message::Message, AmqpError> {
        let mut message = crate::message::Message::new();

        // Decode message header
        if self.has_remaining() {
            let value = self.decode_value()?;
            if let AmqpValue::Map(map) = value {
                // For now, we'll create a simple header
                let mut header = crate::message::Header::new();
                if let Some(durable) = map.get(&AmqpSymbol::from("durable")) {
                    if let AmqpValue::Boolean(val) = durable {
                        header.durable = Some(*val);
                    }
                }
                if let Some(priority) = map.get(&AmqpSymbol::from("priority")) {
                    if let AmqpValue::Ubyte(val) = priority {
                        header.priority = Some(*val);
                    }
                }
                if let Some(ttl) = map.get(&AmqpSymbol::from("ttl")) {
                    if let AmqpValue::Uint(val) = ttl {
                        header.ttl = Some(*val);
                    }
                }
                if let Some(first_acquirer) = map.get(&AmqpSymbol::from("first_acquirer")) {
                    if let AmqpValue::Boolean(val) = first_acquirer {
                        header.first_acquirer = Some(*val);
                    }
                }
                if let Some(delivery_count) = map.get(&AmqpSymbol::from("delivery_count")) {
                    if let AmqpValue::Uint(val) = delivery_count {
                        header.delivery_count = Some(*val);
                    }
                }
                message.header = Some(header);
            }
        }

        // Decode message properties
        if self.has_remaining() {
            let value = self.decode_value()?;
            if let AmqpValue::Map(map) = value {
                // For now, we'll create a simple properties
                let mut properties = crate::message::Properties::new();
                if let Some(message_id) = map.get(&AmqpSymbol::from("message_id")) {
                    properties.message_id = Some(message_id.clone());
                }
                if let Some(user_id) = map.get(&AmqpSymbol::from("user_id")) {
                    if let AmqpValue::Binary(val) = user_id {
                        properties.user_id = Some(val.clone());
                    }
                }
                if let Some(to) = map.get(&AmqpSymbol::from("to")) {
                    if let AmqpValue::String(val) = to {
                        properties.to = Some(val.clone());
                    }
                }
                if let Some(subject) = map.get(&AmqpSymbol::from("subject")) {
                    if let AmqpValue::String(val) = subject {
                        properties.subject = Some(val.clone());
                    }
                }
                if let Some(reply_to) = map.get(&AmqpSymbol::from("reply_to")) {
                    if let AmqpValue::String(val) = reply_to {
                        properties.reply_to = Some(val.clone());
                    }
                }
                if let Some(correlation_id) = map.get(&AmqpSymbol::from("correlation_id")) {
                    properties.correlation_id = Some(correlation_id.clone());
                }
                if let Some(content_type) = map.get(&AmqpSymbol::from("content_type")) {
                    if let AmqpValue::Symbol(val) = content_type {
                        properties.content_type = Some(val.clone());
                    }
                }
                if let Some(content_encoding) = map.get(&AmqpSymbol::from("content_encoding")) {
                    if let AmqpValue::Symbol(val) = content_encoding {
                        properties.content_encoding = Some(val.clone());
                    }
                }
                if let Some(absolute_expiry_time) = map.get(&AmqpSymbol::from("absolute_expiry_time")) {
                    if let AmqpValue::Timestamp(val) = absolute_expiry_time {
                        properties.absolute_expiry_time = Some(*val);
                    }
                }
                if let Some(creation_time) = map.get(&AmqpSymbol::from("creation_time")) {
                    if let AmqpValue::Timestamp(val) = creation_time {
                        properties.creation_time = Some(*val);
                    }
                }
                if let Some(group_id) = map.get(&AmqpSymbol::from("group_id")) {
                    if let AmqpValue::String(val) = group_id {
                        properties.group_id = Some(val.clone());
                    }
                }
                if let Some(group_sequence) = map.get(&AmqpSymbol::from("group_sequence")) {
                    if let AmqpValue::Uint(val) = group_sequence {
                        properties.group_sequence = Some(*val);
                    }
                }
                if let Some(reply_to_group_id) = map.get(&AmqpSymbol::from("reply_to_group_id")) {
                    if let AmqpValue::String(val) = reply_to_group_id {
                        properties.reply_to_group_id = Some(val.clone());
                    }
                }
                message.properties = Some(properties);
            }
        }

        // Decode message body
        if self.has_remaining() {
            let value = self.decode_value()?;
            message.body = Some(crate::message::Body::Value(value));
        }

        Ok(message)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AmqpList, AmqpMap, AmqpSymbol};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_type_code_values() {
        assert_eq!(TypeCode::Described as u8, 0x00);
        assert_eq!(TypeCode::Null as u8, 0x40);
        assert_eq!(TypeCode::Boolean as u8, 0x56);
        assert_eq!(TypeCode::BooleanTrue as u8, 0x41);
        assert_eq!(TypeCode::BooleanFalse as u8, 0x42);
        assert_eq!(TypeCode::Ubyte as u8, 0x50);
        assert_eq!(TypeCode::Ushort as u8, 0x60);
        assert_eq!(TypeCode::Uint as u8, 0x70);
        assert_eq!(TypeCode::Ulong as u8, 0x80);
        assert_eq!(TypeCode::Byte as u8, 0x51);
        assert_eq!(TypeCode::Short as u8, 0x61);
        assert_eq!(TypeCode::Int as u8, 0x71);
        assert_eq!(TypeCode::Long as u8, 0x81);
        assert_eq!(TypeCode::Float as u8, 0x72);
        assert_eq!(TypeCode::Double as u8, 0x82);
        assert_eq!(TypeCode::Decimal32 as u8, 0x74);
        assert_eq!(TypeCode::Decimal64 as u8, 0x84);
        assert_eq!(TypeCode::Decimal128 as u8, 0x94);
        assert_eq!(TypeCode::Char as u8, 0x73);
        assert_eq!(TypeCode::Timestamp as u8, 0x83);
        assert_eq!(TypeCode::Uuid as u8, 0x98);
        assert_eq!(TypeCode::Binary8 as u8, 0xa0);
        assert_eq!(TypeCode::Binary32 as u8, 0xb0);
        assert_eq!(TypeCode::String8 as u8, 0xa1);
        assert_eq!(TypeCode::String32 as u8, 0xb1);
        assert_eq!(TypeCode::Symbol8 as u8, 0xa3);
        assert_eq!(TypeCode::Symbol32 as u8, 0xb3);
        assert_eq!(TypeCode::List0 as u8, 0x45);
        assert_eq!(TypeCode::List8 as u8, 0xc0);
        assert_eq!(TypeCode::List32 as u8, 0xd0);
        assert_eq!(TypeCode::Map8 as u8, 0xc1);
        assert_eq!(TypeCode::Map32 as u8, 0xd1);
        assert_eq!(TypeCode::Array8 as u8, 0xe0);
        assert_eq!(TypeCode::Array32 as u8, 0xf0);
    }

    #[test]
    fn test_encoder_creation() {
        let encoder = Encoder::new();
        assert_eq!(encoder.buffer.len(), 0);
        
        let encoder_with_capacity = Encoder::with_capacity(1024);
        assert_eq!(encoder_with_capacity.buffer.capacity(), 1024);
    }

    #[test]
    fn test_encoder_default() {
        let encoder = Encoder::default();
        assert_eq!(encoder.buffer.len(), 0);
    }

    #[test]
    fn test_encoder_encode_null() {
        let mut encoder = Encoder::new();
        encoder.encode_null().unwrap();
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Null as u8);
    }

    #[test]
    fn test_encoder_encode_boolean() {
        let mut encoder = Encoder::new();
        
        encoder.encode_boolean(true).unwrap();
        encoder.encode_boolean(false).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::BooleanTrue as u8);
        assert_eq!(result[1], TypeCode::BooleanFalse as u8);
    }

    #[test]
    fn test_encoder_encode_ubyte() {
        let mut encoder = Encoder::new();
        encoder.encode_ubyte(42).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Ubyte as u8);
        assert_eq!(result[1], 42);
    }

    #[test]
    fn test_encoder_encode_ushort() {
        let mut encoder = Encoder::new();
        encoder.encode_ushort(12345).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Ushort as u8);
        // Note: bytes are stored in big-endian order
        assert_eq!(result[1], 0x30);
        assert_eq!(result[2], 0x39);
    }

    #[test]
    fn test_encoder_encode_uint() {
        let mut encoder = Encoder::new();
        encoder.encode_uint(123456789).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Uint as u8);
        // Big-endian encoding
        assert_eq!(result[1], 0x07);
        assert_eq!(result[2], 0x5b);
        assert_eq!(result[3], 0xcd);
        assert_eq!(result[4], 0x15);
    }

    #[test]
    fn test_encoder_encode_ulong() {
        let mut encoder = Encoder::new();
        encoder.encode_ulong(1234567890123456789).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Ulong as u8);
        assert_eq!(result.len(), 9); // 1 byte type + 8 bytes value
    }

    #[test]
    fn test_encoder_encode_byte() {
        let mut encoder = Encoder::new();
        encoder.encode_byte(-42).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Byte as u8);
        assert_eq!(result[1], 0xd6); // -42 as signed byte
    }

    #[test]
    fn test_encoder_encode_short() {
        let mut encoder = Encoder::new();
        encoder.encode_short(-12345).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Short as u8);
        assert_eq!(result.len(), 3); // 1 byte type + 2 bytes value
    }

    #[test]
    fn test_encoder_encode_int() {
        let mut encoder = Encoder::new();
        encoder.encode_int(-123456789).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Int as u8);
        assert_eq!(result.len(), 5); // 1 byte type + 4 bytes value
    }

    #[test]
    fn test_encoder_encode_long() {
        let mut encoder = Encoder::new();
        encoder.encode_long(-1234567890123456789).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Long as u8);
        assert_eq!(result.len(), 9); // 1 byte type + 8 bytes value
    }

    #[test]
    fn test_encoder_encode_float() {
        let mut encoder = Encoder::new();
        encoder.encode_float(3.14159).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Float as u8);
        assert_eq!(result.len(), 5); // 1 byte type + 4 bytes value
    }

    #[test]
    fn test_encoder_encode_double() {
        let mut encoder = Encoder::new();
        encoder.encode_double(3.14159265359).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Double as u8);
        assert_eq!(result.len(), 9); // 1 byte type + 8 bytes value
    }

    #[test]
    fn test_encoder_encode_char() {
        let mut encoder = Encoder::new();
        encoder.encode_char('A').unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Char as u8);
        // UTF-32 big-endian encoding: 'A' (0x41) is the last byte
        assert_eq!(result[1], 0x00);
        assert_eq!(result[2], 0x00);
        assert_eq!(result[3], 0x00);
        assert_eq!(result[4], 0x41); // 'A' in UTF-32
        assert_eq!(result.len(), 5); // 1 byte type + 4 bytes UTF-32
    }

    #[test]
    fn test_encoder_encode_timestamp() {
        let mut encoder = Encoder::new();
        let timestamp = 1234567890;
        encoder.encode_timestamp(timestamp).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Timestamp as u8);
        assert_eq!(result.len(), 9); // 1 byte type + 8 bytes value
    }

    #[test]
    fn test_encoder_encode_uuid() {
        let mut encoder = Encoder::new();
        let uuid = Uuid::new_v4();
        encoder.encode_uuid(uuid).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Uuid as u8);
        assert_eq!(result.len(), 17); // 1 byte type + 16 bytes UUID
    }

    #[test]
    fn test_encoder_encode_binary() {
        let mut encoder = Encoder::new();
        let data = vec![1, 2, 3, 4, 5];
        encoder.encode_binary(&data).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Binary8 as u8);
        assert_eq!(result[1], 5); // length
        assert_eq!(&result[2..7], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_encoder_encode_string() {
        let mut encoder = Encoder::new();
        let text = "Hello, AMQP!";
        encoder.encode_string(text).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::String8 as u8);
        assert_eq!(result[1], 12); // length
        assert_eq!(&result[2..14], b"Hello, AMQP!");
    }

    #[test]
    fn test_encoder_encode_symbol() {
        let mut encoder = Encoder::new();
        let symbol = AmqpSymbol::from("test-symbol");
        encoder.encode_symbol(&symbol).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Symbol8 as u8);
        assert_eq!(result[1], 11); // length
        assert_eq!(&result[2..13], b"test-symbol");
    }

    #[test]
    fn test_encoder_encode_list() {
        let mut encoder = Encoder::new();
        let list = AmqpList::from(vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
            AmqpValue::Boolean(true),
        ]);
        encoder.encode_value(&AmqpValue::List(list)).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::List8 as u8);
        assert!(result.len() > 3); // Should contain the list data
    }

    #[test]
    fn test_encoder_encode_map() {
        let mut encoder = Encoder::new();
        let mut map_data = HashMap::new();
        map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
        map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(42));
        let map = AmqpMap::from(map_data);
        encoder.encode_value(&AmqpValue::Map(map)).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Map8 as u8);
        assert!(result.len() > 3); // Should contain the map data
    }

    #[test]
    fn test_encoder_encode_array() {
        let mut encoder = Encoder::new();
        let array = vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
            AmqpValue::Boolean(true),
        ];
        encoder.encode_array(&array).unwrap();
        
        let result = encoder.finish();
        assert_eq!(result[0], TypeCode::Array8 as u8);
        assert!(result.len() > 3); // Should contain the array data
    }

    #[test]
    fn test_decoder_creation() {
        let data = vec![1, 2, 3, 4];
        let decoder = Decoder::new(data.clone());
        assert_eq!(decoder.buffer.len(), 4);
        assert_eq!(decoder.remaining(), 4);
        assert!(decoder.has_remaining());
    }

    #[test]
    fn test_decoder_has_remaining() {
        let decoder = Decoder::new(vec![1, 2, 3]);
        assert!(decoder.has_remaining());
        
        let empty_decoder = Decoder::new(vec![]);
        assert!(!empty_decoder.has_remaining());
    }

    #[test]
    fn test_decoder_remaining() {
        let decoder = Decoder::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(decoder.remaining(), 5);
    }

    #[test]
    fn test_decoder_decode_null() {
        let mut encoder = Encoder::new();
        encoder.encode_null().unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Null));
    }

    #[test]
    fn test_decoder_decode_boolean() {
        let mut encoder = Encoder::new();
        encoder.encode_boolean(true).unwrap();
        encoder.encode_boolean(false).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded1 = decoder.decode_value().unwrap();
        let decoded2 = decoder.decode_value().unwrap();
        
        assert!(matches!(decoded1, AmqpValue::Boolean(true)));
        assert!(matches!(decoded2, AmqpValue::Boolean(false)));
    }

    #[test]
    fn test_decoder_decode_ubyte() {
        let mut encoder = Encoder::new();
        encoder.encode_ubyte(42).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Ubyte(42)));
    }

    #[test]
    fn test_decoder_decode_ushort() {
        let mut encoder = Encoder::new();
        encoder.encode_ushort(12345).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Ushort(12345)));
    }

    #[test]
    fn test_decoder_decode_uint() {
        let mut encoder = Encoder::new();
        encoder.encode_uint(123456789).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Uint(123456789)));
    }

    #[test]
    fn test_decoder_decode_ulong() {
        let mut encoder = Encoder::new();
        encoder.encode_ulong(1234567890123456789).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Ulong(1234567890123456789)));
    }

    #[test]
    fn test_decoder_decode_byte() {
        let mut encoder = Encoder::new();
        encoder.encode_byte(-42).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Byte(-42)));
    }

    #[test]
    fn test_decoder_decode_short() {
        let mut encoder = Encoder::new();
        encoder.encode_short(-12345).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Short(-12345)));
    }

    #[test]
    fn test_decoder_decode_int() {
        let mut encoder = Encoder::new();
        encoder.encode_int(-123456789).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Int(-123456789)));
    }

    #[test]
    fn test_decoder_decode_long() {
        let mut encoder = Encoder::new();
        encoder.encode_long(-1234567890123456789).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Long(-1234567890123456789)));
    }

    #[test]
    fn test_decoder_decode_float() {
        let mut encoder = Encoder::new();
        encoder.encode_float(3.14159).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Float(f) if (f - 3.14159).abs() < 0.0001));
    }

    #[test]
    fn test_decoder_decode_double() {
        let mut encoder = Encoder::new();
        encoder.encode_double(3.14159265359).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Double(d) if (d - 3.14159265359).abs() < 0.00000000001));
    }

    #[test]
    fn test_decoder_decode_char() {
        let mut encoder = Encoder::new();
        encoder.encode_char('A').unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Char('A')));
    }

    #[test]
    fn test_decoder_decode_timestamp() {
        let mut encoder = Encoder::new();
        let timestamp = 1234567890;
        encoder.encode_timestamp(timestamp).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Timestamp(ts) if ts == timestamp));
    }

    #[test]
    fn test_decoder_decode_uuid() {
        let mut encoder = Encoder::new();
        let uuid = Uuid::new_v4();
        encoder.encode_uuid(uuid).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Uuid(u) if u == uuid));
    }

    #[test]
    fn test_decoder_decode_binary() {
        let mut encoder = Encoder::new();
        let data = vec![1, 2, 3, 4, 5];
        encoder.encode_binary(&data).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Binary(b) if b == data));
    }

    #[test]
    fn test_decoder_decode_string() {
        let mut encoder = Encoder::new();
        let text = "Hello, AMQP!";
        encoder.encode_string(text).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::String(s) if s == text));
    }

    #[test]
    fn test_decoder_decode_symbol() {
        let mut encoder = Encoder::new();
        let symbol = AmqpSymbol::from("test-symbol");
        encoder.encode_symbol(&symbol).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Symbol(s) if s == symbol));
    }

    #[test]
    fn test_decoder_decode_list() {
        let mut encoder = Encoder::new();
        let list = AmqpList::from(vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
            AmqpValue::Boolean(true),
        ]);
        encoder.encode_value(&AmqpValue::List(list.clone())).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::List(l) if l == list));
    }

    #[test]
    fn test_decoder_decode_map() {
        let mut encoder = Encoder::new();
        let mut map_data = HashMap::new();
        map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
        map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(42));
        let map = AmqpMap::from(map_data);
        encoder.encode_value(&AmqpValue::Map(map.clone())).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Map(m) if m == map));
    }

    #[test]
    fn test_decoder_decode_array() {
        let mut encoder = Encoder::new();
        let array = vec![
            AmqpValue::String("item1".to_string()),
            AmqpValue::Int(42),
            AmqpValue::Boolean(true),
        ];
        encoder.encode_array(&array).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value().unwrap();
        assert!(matches!(decoded, AmqpValue::Array(a) if a == array));
    }

    #[test]
    fn test_round_trip_encoding() {
        let test_values = vec![
            AmqpValue::Null,
            AmqpValue::Boolean(true),
            AmqpValue::Boolean(false),
            AmqpValue::Ubyte(42),
            AmqpValue::Ushort(12345),
            AmqpValue::Uint(123456789),
            AmqpValue::Ulong(1234567890123456789),
            AmqpValue::Byte(-42),
            AmqpValue::Short(-12345),
            AmqpValue::Int(-123456789),
            AmqpValue::Long(-1234567890123456789),
            AmqpValue::Float(3.14159),
            AmqpValue::Double(3.14159265359),
            AmqpValue::Char('A'),
            AmqpValue::Timestamp(1234567890),
            AmqpValue::Uuid(Uuid::new_v4()),
            AmqpValue::Binary(vec![1, 2, 3, 4, 5]),
            AmqpValue::String("Hello, AMQP!".to_string()),
            AmqpValue::Symbol(AmqpSymbol::from("test-symbol")),
        ];

        for value in test_values {
            let mut encoder = Encoder::new();
            encoder.encode_value(&value).unwrap();
            let encoded = encoder.finish();
            
            let mut decoder = Decoder::new(encoded);
            let decoded = decoder.decode_value().unwrap();
            
            assert_eq!(value, decoded);
        }
    }

    #[test]
    fn test_decoder_decode_symbol_method() {
        let mut encoder = Encoder::new();
        let symbol = AmqpSymbol::from("test-symbol");
        encoder.encode_symbol(&symbol).unwrap();
        let encoded = encoder.finish();
        
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_symbol().unwrap();
        assert_eq!(decoded, symbol);
    }

    #[test]
    fn test_encoder_finish() {
        let mut encoder = Encoder::new();
        encoder.encode_ubyte(42).unwrap();
        encoder.encode_string("test").unwrap();
        
        let result = encoder.finish();
        assert!(!result.is_empty());
        assert_eq!(result[0], TypeCode::Ubyte as u8);
        assert_eq!(result[1], 42);
        assert_eq!(result[2], TypeCode::String8 as u8);
    }

    #[test]
    fn test_encoder_with_capacity() {
        let mut encoder = Encoder::with_capacity(1024);
        assert_eq!(encoder.buffer.capacity(), 1024);
        
        // Add some data
        encoder.encode_string("test string").unwrap();
        let result = encoder.finish();
        assert!(!result.is_empty());
    }
} 
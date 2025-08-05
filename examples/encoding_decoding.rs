//! AMQP 1.0 Encoding/Decoding Example
//!
//! This example demonstrates how to encode and decode AMQP values using the codec.

use dumq_amqp::codec::{Encoder, Decoder};
use dumq_amqp::types::{AmqpValue, AmqpSymbol, AmqpList, AmqpMap};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("dumq_amqp Encoding/Decoding Example");
    println!("===============================");

    // Test basic value encoding/decoding
    println!("\n1. Basic Value Encoding/Decoding:");
    test_basic_values()?;

    // Test complex value encoding/decoding
    println!("\n2. Complex Value Encoding/Decoding:");
    test_complex_values()?;

    // Test multiple values encoding/decoding
    println!("\n3. Multiple Values Encoding/Decoding:");
    test_multiple_values()?;

    println!("\nEncoding/Decoding example completed successfully!");
    Ok(())
}

fn test_basic_values() -> Result<(), Box<dyn std::error::Error>> {
    let values = vec![
        AmqpValue::String("Hello, AMQP!".to_string()),
        AmqpValue::Int(42),
        AmqpValue::Boolean(true),
        AmqpValue::Double(3.14159),
        AmqpValue::Uuid(uuid::Uuid::new_v4()),
        AmqpValue::Binary(vec![1, 2, 3, 4, 5]),
    ];

    for value in values {
        println!("  Original: {:?}", value);
        
        // Encode the value
        let mut encoder = Encoder::new();
        encoder.encode_value(&value)?;
        let encoded = encoder.finish();
        
        println!("  Encoded: {:?} bytes", encoded.len());
        
        // Decode the value
        let mut decoder = Decoder::new(encoded);
        let decoded = decoder.decode_value()?;
        
        println!("  Decoded: {:?}", decoded);
        println!("  Match: {}", value == decoded);
        println!();
    }

    Ok(())
}

fn test_complex_values() -> Result<(), Box<dyn std::error::Error>> {
    // Test AmqpList
    println!("  Testing AmqpList:");
    let list = AmqpList::from(vec![
        AmqpValue::String("item1".to_string()),
        AmqpValue::Int(42),
        AmqpValue::Boolean(true),
    ]);
    let list_value = AmqpValue::List(list);
    
    println!("    Original: {:?}", list_value);
    
    let mut encoder = Encoder::new();
    encoder.encode_value(&list_value)?;
    let encoded = encoder.finish();
    
    println!("    Encoded: {:?} bytes", encoded.len());
    
    let mut decoder = Decoder::new(encoded);
    let decoded = decoder.decode_value()?;
    
    println!("    Decoded: {:?}", decoded);
    println!("    Match: {}", list_value == decoded);

    // Test AmqpMap
    println!("  Testing AmqpMap:");
    let mut map_data = HashMap::new();
    map_data.insert(AmqpSymbol::from("key1"), AmqpValue::String("value1".to_string()));
    map_data.insert(AmqpSymbol::from("key2"), AmqpValue::Int(123));
    map_data.insert(AmqpSymbol::from("key3"), AmqpValue::Boolean(false));
    let map = AmqpMap::from(map_data);
    let map_value = AmqpValue::Map(map);
    
    println!("    Original: {:?}", map_value);
    
    let mut encoder = Encoder::new();
    encoder.encode_value(&map_value)?;
    let encoded = encoder.finish();
    
    println!("    Encoded: {:?} bytes", encoded.len());
    
    let mut decoder = Decoder::new(encoded);
    let decoded = decoder.decode_value()?;
    
    println!("    Decoded: {:?}", decoded);
    println!("    Match: {}", map_value == decoded);

    Ok(())
}

fn test_multiple_values() -> Result<(), Box<dyn std::error::Error>> {
    let values = vec![
        AmqpValue::String("First value".to_string()),
        AmqpValue::Int(100),
        AmqpValue::Boolean(false),
        AmqpValue::Double(2.71828),
    ];

    println!("  Encoding multiple values...");
    
    // Encode multiple values
    let mut encoder = Encoder::new();
    for value in &values {
        encoder.encode_value(value)?;
    }
    let encoded = encoder.finish();
    
    println!("  Total encoded size: {:?} bytes", encoded.len());
    
    // Decode multiple values
    let mut decoder = Decoder::new(encoded);
    let mut decoded_values = Vec::new();
    
    while decoder.has_remaining() {
        match decoder.decode_value() {
            Ok(value) => decoded_values.push(value),
            Err(e) => {
                println!("  Decoding error: {}", e);
                break;
            }
        }
    }
    
    println!("  Decoded {} values", decoded_values.len());
    
    // Verify all values match
    let mut all_match = true;
    for (i, (original, decoded)) in values.iter().zip(decoded_values.iter()).enumerate() {
        if original != decoded {
            println!("  Value {} mismatch: {:?} != {:?}", i, original, decoded);
            all_match = false;
        }
    }
    
    if all_match {
        println!("  All values decoded correctly!");
    }

    Ok(())
} 
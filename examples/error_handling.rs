//! AMQP 1.0 Error Handling Example
//!
//! This example demonstrates how to handle various types of AMQP errors.

use dumq_amqp::prelude::*;
use dumq_amqp::error::AmqpError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("dumq_amqp Condition Handling Example");
    println!("================================");

    // 1. Test all AMQP conditions (success and error)
    println!("\n1. Testing All AMQP Conditions:");
    test_all_amqp_conditions()?;

    // 2. Test condition classification
    println!("\n2. Testing Condition Classification:");
    test_condition_classification()?;

    // 3. Test condition conversion
    println!("\n3. Testing Condition Conversion:");
    test_condition_conversion()?;

    // 4. Test condition categories
    println!("\n4. Testing Condition Categories:");
    test_condition_categories()?;

    // 5. Test error handling with conditions
    println!("\n5. Testing Error Handling with Conditions:");
    test_error_handling_with_conditions()?;

    println!("\nCondition handling example completed successfully!");
    Ok(())
}

/// Test all AMQP conditions (success and error)
fn test_all_amqp_conditions() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing all AMQP conditions with their codes...");
    
    let conditions = vec![
        // Success conditions (200-series)
        AmqpCondition::Ok,
        AmqpCondition::Accepted,
        AmqpCondition::Released,
        AmqpCondition::Modified,
        
        // Error conditions (300-500 series)
        AmqpCondition::AmqpErrorConnectionForced,
        AmqpCondition::AmqpErrorFramingError,
        AmqpCondition::AmqpErrorConnectionRedirect,
        AmqpCondition::AmqpErrorWindowViolation,
        AmqpCondition::AmqpErrorErrantLink,
        AmqpCondition::AmqpErrorHandleInUse,
        AmqpCondition::AmqpErrorMessageSizeExceeded,
        AmqpCondition::AmqpErrorLinkRedirect,
        AmqpCondition::AmqpErrorTransferRefused,
        AmqpCondition::AmqpErrorStolen,
        AmqpCondition::AmqpErrorResourceDeleted,
        AmqpCondition::AmqpErrorResourceLimitExceeded,
        AmqpCondition::AmqpErrorResourceLocked,
        AmqpCondition::AmqpErrorPreconditionFailed,
        AmqpCondition::AmqpErrorResourceNameCollision,
        AmqpCondition::AmqpErrorUnauthorizedAccess,
        AmqpCondition::AmqpErrorNotAllowed,
        AmqpCondition::AmqpErrorNotImplemented,
        AmqpCondition::AmqpErrorNotModified,
        AmqpCondition::AmqpErrorDecodeError,
        AmqpCondition::AmqpErrorInvalidField,
        AmqpCondition::AmqpErrorNotAccepted,
        AmqpCondition::AmqpErrorRejected,
        AmqpCondition::AmqpErrorInternalError,
        AmqpCondition::AmqpErrorIllegalState,
        
        // Custom conditions
        AmqpCondition::Custom("my-custom-success".to_string()),
        AmqpCondition::Custom("my-custom-error".to_string()),
    ];

    for condition in conditions {
        let status = if condition.is_success() { "SUCCESS" } else { "ERROR" };
        println!("    {:?} -> {} (code: {} num: {})", 
                 condition, status, condition.as_str(), condition.code_num());
    }

    Ok(())
}

/// Test condition classification
fn test_condition_classification() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing condition classification...");
    
    let success_conditions = vec![
        AmqpCondition::Ok,
        AmqpCondition::Accepted,
        AmqpCondition::Released,
        AmqpCondition::Modified,
    ];

    let error_conditions = vec![
        AmqpCondition::AmqpErrorConnectionForced,
        AmqpCondition::AmqpErrorUnauthorizedAccess,
        AmqpCondition::AmqpErrorResourceDeleted,
        AmqpCondition::AmqpErrorNotImplemented,
    ];

    println!("  Success Conditions:");
    for condition in success_conditions {
        println!("    {:?} -> is_success: {}, is_error: {}", 
                 condition, condition.is_success(), condition.is_error());
    }

    println!("  Error Conditions:");
    for condition in error_conditions {
        println!("    {:?} -> is_success: {}, is_error: {}", 
                 condition, condition.is_success(), condition.is_error());
    }

    Ok(())
}

/// Test condition conversion
fn test_condition_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing condition conversion...");
    
    let test_strings = vec![
        "amqp:ok",
        "amqp:accepted", 
        "amqp:connection:forced",
        "amqp:access:unauthorized",
        "amqp:resource:deleted",
        "amqp:not-implemented",
        "my-custom-condition",
    ];

    for s in test_strings {
        let condition: AmqpCondition = s.into();
        println!("    '{}' -> {:?} (code: {} num: {})", 
                 s, condition, condition.as_str(), condition.code_num());
    }

    Ok(())
}

/// Test condition categories
fn test_condition_categories() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing condition categories...");
    
    let test_conditions = vec![
        (AmqpCondition::Ok, "Success condition"),
        (AmqpCondition::AmqpErrorConnectionForced, "Connection error"),
        (AmqpCondition::AmqpErrorWindowViolation, "Session error"),
        (AmqpCondition::AmqpErrorMessageSizeExceeded, "Link error"),
        (AmqpCondition::AmqpErrorResourceDeleted, "Resource error"),
        (AmqpCondition::AmqpErrorUnauthorizedAccess, "Access error"),
        (AmqpCondition::AmqpErrorDecodeError, "Content error"),
        (AmqpCondition::AmqpErrorInternalError, "Internal error"),
        (AmqpCondition::Custom("custom".to_string()), "Custom condition"),
    ];

    for (condition, description) in test_conditions {
        println!("    {:?} -> Category: {} ({})", 
                 condition, condition.category(), description);
    }

    Ok(())
}

/// Test error handling with conditions
fn test_error_handling_with_conditions() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing error handling with conditions...");
    
    let test_cases = vec![
        (AmqpCondition::Ok, "Operation completed successfully"),
        (AmqpCondition::Accepted, "Message was accepted"),
        (AmqpCondition::AmqpErrorConnectionForced, "Connection was forced to close"),
        (AmqpCondition::AmqpErrorUnauthorizedAccess, "Access denied"),
        (AmqpCondition::AmqpErrorResourceDeleted, "Resource not found"),
        (AmqpCondition::AmqpErrorNotImplemented, "Feature not implemented"),
    ];

    for (condition, description) in test_cases {
        if condition.is_error() {
            let error = AmqpError::amqp_protocol(condition.clone(), description);
            println!("    ERROR: {:?} -> {} (code: {} num: {})", 
                     condition, error, error.error_code(), error.error_code_num());
        } else {
            println!("    SUCCESS: {:?} -> {} (code: {} num: {})", 
                     condition, description, condition.as_str(), condition.code_num());
        }
    }

    Ok(())
} 
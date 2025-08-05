//! AMQP 1.0 Condition System
//! 
//! This module provides the condition system for AMQP 1.0, including both
//! success and error conditions with their corresponding numeric codes.

use serde::{Deserialize, Serialize};

/// AMQP 1.0 Condition Codes (Success and Error)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmqpCondition {
    // Success conditions (200-series)
    Ok,
    Accepted,
    Released,
    Modified,
    
    // Error conditions (300-500 series)
    AmqpErrorConnectionForced,
    AmqpErrorFramingError,
    AmqpErrorConnectionRedirect,
    AmqpErrorWindowViolation,
    AmqpErrorErrantLink,
    AmqpErrorHandleInUse,
    AmqpErrorDetachForced,
    AmqpErrorTransferLimitExceeded,
    AmqpErrorMessageSizeExceeded,
    AmqpErrorLinkRedirect,
    AmqpErrorTransferRefused,
    AmqpErrorStolen,
    AmqpErrorResourceDeleted,
    AmqpErrorResourceLimitExceeded,
    AmqpErrorResourceLocked,
    AmqpErrorPreconditionFailed,
    AmqpErrorResourceNameCollision,
    AmqpErrorUnauthorizedAccess,
    AmqpErrorNotAllowed,
    AmqpErrorNotImplemented,
    AmqpErrorNotModified,
    AmqpErrorDecodeError,
    AmqpErrorInvalidField,
    AmqpErrorNotAccepted,
    AmqpErrorRejected,
    AmqpErrorInternalError,
    AmqpErrorIllegalState,
    
    // Custom conditions
    Custom(String),
}

impl AmqpCondition {
    /// Get the string representation of the condition
    pub fn as_str(&self) -> &str {
        match self {
            // Success conditions
            AmqpCondition::Ok => "amqp:ok",
            AmqpCondition::Accepted => "amqp:accepted",
            AmqpCondition::Released => "amqp:released",
            AmqpCondition::Modified => "amqp:modified",
            
            // Error conditions
            AmqpCondition::AmqpErrorConnectionForced => "amqp:connection:forced",
            AmqpCondition::AmqpErrorFramingError => "amqp:connection:framing-error",
            AmqpCondition::AmqpErrorConnectionRedirect => "amqp:connection:redirect",
            AmqpCondition::AmqpErrorWindowViolation => "amqp:session:window-violation",
            AmqpCondition::AmqpErrorErrantLink => "amqp:session:errant-link",
            AmqpCondition::AmqpErrorHandleInUse => "amqp:session:handle-in-use",
            AmqpCondition::AmqpErrorDetachForced => "amqp:session:detach-forced",
            AmqpCondition::AmqpErrorTransferLimitExceeded => "amqp:session:transfer-limit-exceeded",
            AmqpCondition::AmqpErrorMessageSizeExceeded => "amqp:link:message-size-exceeded",
            AmqpCondition::AmqpErrorLinkRedirect => "amqp:link:redirect",
            AmqpCondition::AmqpErrorTransferRefused => "amqp:link:transfer-refused",
            AmqpCondition::AmqpErrorStolen => "amqp:link:stolen",
            AmqpCondition::AmqpErrorResourceDeleted => "amqp:resource:deleted",
            AmqpCondition::AmqpErrorResourceLimitExceeded => "amqp:resource:limit-exceeded",
            AmqpCondition::AmqpErrorResourceLocked => "amqp:resource:locked",
            AmqpCondition::AmqpErrorPreconditionFailed => "amqp:resource:precondition-failed",
            AmqpCondition::AmqpErrorResourceNameCollision => "amqp:resource:name-collision",
            AmqpCondition::AmqpErrorUnauthorizedAccess => "amqp:access:unauthorized",
            AmqpCondition::AmqpErrorNotAllowed => "amqp:access:not-allowed",
            AmqpCondition::AmqpErrorNotImplemented => "amqp:not-implemented",
            AmqpCondition::AmqpErrorNotModified => "amqp:not-modified",
            AmqpCondition::AmqpErrorDecodeError => "amqp:decode-error",
            AmqpCondition::AmqpErrorInvalidField => "amqp:invalid-field",
            AmqpCondition::AmqpErrorNotAccepted => "amqp:not-accepted",
            AmqpCondition::AmqpErrorRejected => "amqp:rejected",
            AmqpCondition::AmqpErrorInternalError => "amqp:internal-error",
            AmqpCondition::AmqpErrorIllegalState => "amqp:illegal-state",
            AmqpCondition::Custom(s) => s.as_str(),
        }
    }

    /// Return a numeric code for the AMQP condition
    pub fn code_num(&self) -> u16 {
        match self {
            // 200-series: Success
            AmqpCondition::Ok => 200,
            AmqpCondition::Accepted => 202,
            AmqpCondition::Released => 200,
            AmqpCondition::Modified => 200,
            
            // 300-series: Soft/Client Errors
            AmqpCondition::AmqpErrorMessageSizeExceeded => 311,
            AmqpCondition::AmqpErrorDecodeError => 320,
            AmqpCondition::AmqpErrorInvalidField => 320,
            AmqpCondition::AmqpErrorNotAccepted => 320,
            
            // 400-series: Channel/Connection Errors
            AmqpCondition::AmqpErrorUnauthorizedAccess => 403,
            AmqpCondition::AmqpErrorNotAllowed => 403,
            AmqpCondition::AmqpErrorResourceDeleted => 404,
            AmqpCondition::AmqpErrorResourceNameCollision => 405,
            AmqpCondition::AmqpErrorResourceLocked => 406,
            
            // 500-series: Server Errors
            AmqpCondition::AmqpErrorNotImplemented => 530,
            AmqpCondition::AmqpErrorInternalError => 500,
            AmqpCondition::AmqpErrorIllegalState => 500,
            
            // 기타 에러는 500
            _ => 500,
        }
    }

    /// Check if this is a success condition
    pub fn is_success(&self) -> bool {
        matches!(self, 
            AmqpCondition::Ok | 
            AmqpCondition::Accepted | 
            AmqpCondition::Released | 
            AmqpCondition::Modified
        )
    }

    /// Check if this is an error condition
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }

    /// Get the category of this condition
    pub fn category(&self) -> ConditionCategory {
        match self {
            // Success conditions
            AmqpCondition::Ok | AmqpCondition::Accepted | 
            AmqpCondition::Released | AmqpCondition::Modified => ConditionCategory::Success,
            
            // Connection errors
            AmqpCondition::AmqpErrorConnectionForced | 
            AmqpCondition::AmqpErrorFramingError | 
            AmqpCondition::AmqpErrorConnectionRedirect => ConditionCategory::Connection,
            
            // Session errors
            AmqpCondition::AmqpErrorWindowViolation | 
            AmqpCondition::AmqpErrorErrantLink | 
            AmqpCondition::AmqpErrorHandleInUse | 
            AmqpCondition::AmqpErrorDetachForced | 
            AmqpCondition::AmqpErrorTransferLimitExceeded => ConditionCategory::Session,
            
            // Link errors
            AmqpCondition::AmqpErrorMessageSizeExceeded | 
            AmqpCondition::AmqpErrorLinkRedirect | 
            AmqpCondition::AmqpErrorTransferRefused | 
            AmqpCondition::AmqpErrorStolen => ConditionCategory::Link,
            
            // Resource errors
            AmqpCondition::AmqpErrorResourceDeleted | 
            AmqpCondition::AmqpErrorResourceLimitExceeded | 
            AmqpCondition::AmqpErrorResourceLocked | 
            AmqpCondition::AmqpErrorPreconditionFailed | 
            AmqpCondition::AmqpErrorResourceNameCollision => ConditionCategory::Resource,
            
            // Access errors
            AmqpCondition::AmqpErrorUnauthorizedAccess | 
            AmqpCondition::AmqpErrorNotAllowed => ConditionCategory::Access,
            
            // Content errors
            AmqpCondition::AmqpErrorDecodeError | 
            AmqpCondition::AmqpErrorInvalidField | 
            AmqpCondition::AmqpErrorNotAccepted | 
            AmqpCondition::AmqpErrorRejected => ConditionCategory::Content,
            
            // Internal errors
            AmqpCondition::AmqpErrorInternalError | 
            AmqpCondition::AmqpErrorIllegalState | 
            AmqpCondition::AmqpErrorNotImplemented | 
            AmqpCondition::AmqpErrorNotModified => ConditionCategory::Internal,
            
            // Custom conditions
            AmqpCondition::Custom(_) => ConditionCategory::Custom,
        }
    }
}

impl std::fmt::Display for AmqpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for AmqpCondition {
    fn from(s: &str) -> Self {
        match s {
            // Success conditions
            "amqp:ok" => AmqpCondition::Ok,
            "amqp:accepted" => AmqpCondition::Accepted,
            "amqp:released" => AmqpCondition::Released,
            "amqp:modified" => AmqpCondition::Modified,
            
            // Error conditions
            "amqp:connection:forced" => AmqpCondition::AmqpErrorConnectionForced,
            "amqp:connection:framing-error" => AmqpCondition::AmqpErrorFramingError,
            "amqp:connection:redirect" => AmqpCondition::AmqpErrorConnectionRedirect,
            "amqp:session:window-violation" => AmqpCondition::AmqpErrorWindowViolation,
            "amqp:session:errant-link" => AmqpCondition::AmqpErrorErrantLink,
            "amqp:session:handle-in-use" => AmqpCondition::AmqpErrorHandleInUse,
            "amqp:session:detach-forced" => AmqpCondition::AmqpErrorDetachForced,
            "amqp:session:transfer-limit-exceeded" => AmqpCondition::AmqpErrorTransferLimitExceeded,
            "amqp:link:message-size-exceeded" => AmqpCondition::AmqpErrorMessageSizeExceeded,
            "amqp:link:redirect" => AmqpCondition::AmqpErrorLinkRedirect,
            "amqp:link:transfer-refused" => AmqpCondition::AmqpErrorTransferRefused,
            "amqp:link:stolen" => AmqpCondition::AmqpErrorStolen,
            "amqp:resource:deleted" => AmqpCondition::AmqpErrorResourceDeleted,
            "amqp:resource:limit-exceeded" => AmqpCondition::AmqpErrorResourceLimitExceeded,
            "amqp:resource:locked" => AmqpCondition::AmqpErrorResourceLocked,
            "amqp:resource:precondition-failed" => AmqpCondition::AmqpErrorPreconditionFailed,
            "amqp:resource:name-collision" => AmqpCondition::AmqpErrorResourceNameCollision,
            "amqp:access:unauthorized" => AmqpCondition::AmqpErrorUnauthorizedAccess,
            "amqp:access:not-allowed" => AmqpCondition::AmqpErrorNotAllowed,
            "amqp:not-implemented" => AmqpCondition::AmqpErrorNotImplemented,
            "amqp:not-modified" => AmqpCondition::AmqpErrorNotModified,
            "amqp:decode-error" => AmqpCondition::AmqpErrorDecodeError,
            "amqp:invalid-field" => AmqpCondition::AmqpErrorInvalidField,
            "amqp:not-accepted" => AmqpCondition::AmqpErrorNotAccepted,
            "amqp:rejected" => AmqpCondition::AmqpErrorRejected,
            "amqp:internal-error" => AmqpCondition::AmqpErrorInternalError,
            "amqp:illegal-state" => AmqpCondition::AmqpErrorIllegalState,
            _ => AmqpCondition::Custom(s.to_string()),
        }
    }
}

/// Categories of AMQP conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionCategory {
    Success,
    Connection,
    Session,
    Link,
    Resource,
    Access,
    Content,
    Internal,
    Custom,
}

impl std::fmt::Display for ConditionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionCategory::Success => write!(f, "Success"),
            ConditionCategory::Connection => write!(f, "Connection"),
            ConditionCategory::Session => write!(f, "Session"),
            ConditionCategory::Link => write!(f, "Link"),
            ConditionCategory::Resource => write!(f, "Resource"),
            ConditionCategory::Access => write!(f, "Access"),
            ConditionCategory::Content => write!(f, "Content"),
            ConditionCategory::Internal => write!(f, "Internal"),
            ConditionCategory::Custom => write!(f, "Custom"),
        }
    }
}

// Backward compatibility - keep AmqpErrorCondition as an alias
pub type AmqpErrorCondition = AmqpCondition; 
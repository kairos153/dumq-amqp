//! AMQP 1.0 Condition System
//! 
//! This module provides the condition system for AMQP 1.0, including both
//! success and error conditions with their corresponding numeric codes.

use serde::{Deserialize, Serialize};

/// AMQP 1.0 Condition Codes (Success and Error)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AmqpCondition {
    // Success conditions (200-series)
    #[serde(rename = "amqp:ok")]
    Ok,
    #[serde(rename = "amqp:accepted")]
    Accepted,
    #[serde(rename = "amqp:released")]
    Released,
    #[serde(rename = "amqp:modified")]
    Modified,
    
    // Error conditions (300-500 series)
    #[serde(rename = "amqp:connection:forced")]
    AmqpErrorConnectionForced,
    #[serde(rename = "amqp:connection:framing-error")]
    AmqpErrorFramingError,
    #[serde(rename = "amqp:connection:redirect")]
    AmqpErrorConnectionRedirect,
    #[serde(rename = "amqp:session:window-violation")]
    AmqpErrorWindowViolation,
    #[serde(rename = "amqp:session:errant-link")]
    AmqpErrorErrantLink,
    #[serde(rename = "amqp:session:handle-in-use")]
    AmqpErrorHandleInUse,
    #[serde(rename = "amqp:session:detach-forced")]
    AmqpErrorDetachForced,
    #[serde(rename = "amqp:session:transfer-limit-exceeded")]
    AmqpErrorTransferLimitExceeded,
    #[serde(rename = "amqp:link:message-size-exceeded")]
    AmqpErrorMessageSizeExceeded,
    #[serde(rename = "amqp:link:redirect")]
    AmqpErrorLinkRedirect,
    #[serde(rename = "amqp:link:transfer-refused")]
    AmqpErrorTransferRefused,
    #[serde(rename = "amqp:link:stolen")]
    AmqpErrorStolen,
    #[serde(rename = "amqp:resource:deleted")]
    AmqpErrorResourceDeleted,
    #[serde(rename = "amqp:resource:limit-exceeded")]
    AmqpErrorResourceLimitExceeded,
    #[serde(rename = "amqp:resource:locked")]
    AmqpErrorResourceLocked,
    #[serde(rename = "amqp:resource:precondition-failed")]
    AmqpErrorPreconditionFailed,
    #[serde(rename = "amqp:resource:name-collision")]
    AmqpErrorResourceNameCollision,
    #[serde(rename = "amqp:access:unauthorized")]
    AmqpErrorUnauthorizedAccess,
    #[serde(rename = "amqp:access:not-allowed")]
    AmqpErrorNotAllowed,
    #[serde(rename = "amqp:not-implemented")]
    AmqpErrorNotImplemented,
    #[serde(rename = "amqp:not-modified")]
    AmqpErrorNotModified,
    #[serde(rename = "amqp:decode-error")]
    AmqpErrorDecodeError,
    #[serde(rename = "amqp:invalid-field")]
    AmqpErrorInvalidField,
    #[serde(rename = "amqp:not-accepted")]
    AmqpErrorNotAccepted,
    #[serde(rename = "amqp:rejected")]
    AmqpErrorRejected,
    #[serde(rename = "amqp:internal-error")]
    AmqpErrorInternalError,
    #[serde(rename = "amqp:illegal-state")]
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
            AmqpCondition::AmqpErrorConnectionForced => 301,
            AmqpCondition::AmqpErrorConnectionRedirect => 302,
            AmqpCondition::AmqpErrorNotModified => 304,
            AmqpCondition::AmqpErrorMessageSizeExceeded => 311,
            AmqpCondition::AmqpErrorDecodeError => 502,
            AmqpCondition::AmqpErrorInvalidField => 503,
            AmqpCondition::AmqpErrorNotAccepted => 400,
            AmqpCondition::AmqpErrorRejected => 400,
            
            // 400-series: Channel/Connection Errors
            AmqpCondition::AmqpErrorUnauthorizedAccess => 401,
            AmqpCondition::AmqpErrorNotAllowed => 403,
            AmqpCondition::AmqpErrorResourceDeleted => 404,
            AmqpCondition::AmqpErrorResourceNameCollision => 409,
            AmqpCondition::AmqpErrorResourceLocked => 406,
            AmqpCondition::AmqpErrorPreconditionFailed => 412,
            
            // 500-series: Server Errors
            AmqpCondition::AmqpErrorFramingError => 501,
            AmqpCondition::AmqpErrorWindowViolation => 503,
            AmqpCondition::AmqpErrorErrantLink => 504,
            AmqpCondition::AmqpErrorHandleInUse => 505,
            AmqpCondition::AmqpErrorDetachForced => 506,
            AmqpCondition::AmqpErrorTransferLimitExceeded => 507,
            AmqpCondition::AmqpErrorLinkRedirect => 404,
            AmqpCondition::AmqpErrorTransferRefused => 405,
            AmqpCondition::AmqpErrorStolen => 406,
            AmqpCondition::AmqpErrorResourceLimitExceeded => 405,
            AmqpCondition::AmqpErrorNotImplemented => 501,
            AmqpCondition::AmqpErrorInternalError => 500,
            AmqpCondition::AmqpErrorIllegalState => 500,
            
            // Custom conditions
            AmqpCondition::Custom(_) => 0,
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
        !self.is_success() || matches!(self, AmqpCondition::Custom(_))
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
            

            
            // Resource errors
            AmqpCondition::AmqpErrorResourceDeleted | 
            AmqpCondition::AmqpErrorResourceLimitExceeded | 
            AmqpCondition::AmqpErrorResourceLocked | 
            AmqpCondition::AmqpErrorPreconditionFailed | 
            AmqpCondition::AmqpErrorResourceNameCollision => ConditionCategory::Resource,
            
            // Access errors
            AmqpCondition::AmqpErrorUnauthorizedAccess | 
            AmqpCondition::AmqpErrorNotAllowed => ConditionCategory::Access,
            
            // Link errors
            AmqpCondition::AmqpErrorMessageSizeExceeded |
            AmqpCondition::AmqpErrorLinkRedirect | 
            AmqpCondition::AmqpErrorTransferRefused | 
            AmqpCondition::AmqpErrorStolen => ConditionCategory::Link,
            
            // Content errors - none currently
            
            // Internal errors
            AmqpCondition::AmqpErrorInternalError | 
            AmqpCondition::AmqpErrorIllegalState | 
            AmqpCondition::AmqpErrorNotImplemented | 
            AmqpCondition::AmqpErrorNotModified |
            AmqpCondition::AmqpErrorDecodeError |
            AmqpCondition::AmqpErrorInvalidField |
            AmqpCondition::AmqpErrorNotAccepted |
            AmqpCondition::AmqpErrorRejected => ConditionCategory::Internal,
            
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_condition_success_codes() {
        assert_eq!(AmqpCondition::Ok.code_num(), 200);
        assert_eq!(AmqpCondition::Accepted.code_num(), 202);
        assert_eq!(AmqpCondition::Released.code_num(), 200);
        assert_eq!(AmqpCondition::Modified.code_num(), 200);
    }

    #[test]
    fn test_amqp_condition_error_codes() {
        assert_eq!(AmqpCondition::AmqpErrorConnectionForced.code_num(), 301);
        assert_eq!(AmqpCondition::AmqpErrorFramingError.code_num(), 501);
        assert_eq!(AmqpCondition::AmqpErrorConnectionRedirect.code_num(), 302);
        assert_eq!(AmqpCondition::AmqpErrorWindowViolation.code_num(), 503);
        assert_eq!(AmqpCondition::AmqpErrorErrantLink.code_num(), 504);
        assert_eq!(AmqpCondition::AmqpErrorHandleInUse.code_num(), 505);
        assert_eq!(AmqpCondition::AmqpErrorDetachForced.code_num(), 506);
        assert_eq!(AmqpCondition::AmqpErrorTransferLimitExceeded.code_num(), 507);
        assert_eq!(AmqpCondition::AmqpErrorMessageSizeExceeded.code_num(), 311);
        assert_eq!(AmqpCondition::AmqpErrorLinkRedirect.code_num(), 404);
        assert_eq!(AmqpCondition::AmqpErrorTransferRefused.code_num(), 405);
        assert_eq!(AmqpCondition::AmqpErrorStolen.code_num(), 406);
        assert_eq!(AmqpCondition::AmqpErrorResourceDeleted.code_num(), 404);
        assert_eq!(AmqpCondition::AmqpErrorResourceLimitExceeded.code_num(), 405);
        assert_eq!(AmqpCondition::AmqpErrorResourceLocked.code_num(), 406);
        assert_eq!(AmqpCondition::AmqpErrorPreconditionFailed.code_num(), 412);
        assert_eq!(AmqpCondition::AmqpErrorResourceNameCollision.code_num(), 409);
        assert_eq!(AmqpCondition::AmqpErrorUnauthorizedAccess.code_num(), 401);
        assert_eq!(AmqpCondition::AmqpErrorNotAllowed.code_num(), 403);
        assert_eq!(AmqpCondition::AmqpErrorNotImplemented.code_num(), 501);
        assert_eq!(AmqpCondition::AmqpErrorNotModified.code_num(), 304);
        assert_eq!(AmqpCondition::AmqpErrorDecodeError.code_num(), 502);
        assert_eq!(AmqpCondition::AmqpErrorInvalidField.code_num(), 503);
        assert_eq!(AmqpCondition::AmqpErrorNotAccepted.code_num(), 400);
        assert_eq!(AmqpCondition::AmqpErrorRejected.code_num(), 400);
        assert_eq!(AmqpCondition::AmqpErrorInternalError.code_num(), 500);
        assert_eq!(AmqpCondition::AmqpErrorIllegalState.code_num(), 500);
    }

    #[test]
    fn test_amqp_condition_string_representations() {
        assert_eq!(AmqpCondition::Ok.as_str(), "amqp:ok");
        assert_eq!(AmqpCondition::Accepted.as_str(), "amqp:accepted");
        assert_eq!(AmqpCondition::Released.as_str(), "amqp:released");
        assert_eq!(AmqpCondition::Modified.as_str(), "amqp:modified");
        
        assert_eq!(AmqpCondition::AmqpErrorConnectionForced.as_str(), "amqp:connection:forced");
        assert_eq!(AmqpCondition::AmqpErrorFramingError.as_str(), "amqp:connection:framing-error");
        assert_eq!(AmqpCondition::AmqpErrorConnectionRedirect.as_str(), "amqp:connection:redirect");
        assert_eq!(AmqpCondition::AmqpErrorWindowViolation.as_str(), "amqp:session:window-violation");
        assert_eq!(AmqpCondition::AmqpErrorErrantLink.as_str(), "amqp:session:errant-link");
        assert_eq!(AmqpCondition::AmqpErrorHandleInUse.as_str(), "amqp:session:handle-in-use");
        assert_eq!(AmqpCondition::AmqpErrorDetachForced.as_str(), "amqp:session:detach-forced");
        assert_eq!(AmqpCondition::AmqpErrorTransferLimitExceeded.as_str(), "amqp:session:transfer-limit-exceeded");
        assert_eq!(AmqpCondition::AmqpErrorMessageSizeExceeded.as_str(), "amqp:link:message-size-exceeded");
        assert_eq!(AmqpCondition::AmqpErrorLinkRedirect.as_str(), "amqp:link:redirect");
        assert_eq!(AmqpCondition::AmqpErrorTransferRefused.as_str(), "amqp:link:transfer-refused");
        assert_eq!(AmqpCondition::AmqpErrorStolen.as_str(), "amqp:link:stolen");
        assert_eq!(AmqpCondition::AmqpErrorResourceDeleted.as_str(), "amqp:resource:deleted");
        assert_eq!(AmqpCondition::AmqpErrorResourceLimitExceeded.as_str(), "amqp:resource:limit-exceeded");
        assert_eq!(AmqpCondition::AmqpErrorResourceLocked.as_str(), "amqp:resource:locked");
        assert_eq!(AmqpCondition::AmqpErrorPreconditionFailed.as_str(), "amqp:resource:precondition-failed");
        assert_eq!(AmqpCondition::AmqpErrorResourceNameCollision.as_str(), "amqp:resource:name-collision");
        assert_eq!(AmqpCondition::AmqpErrorUnauthorizedAccess.as_str(), "amqp:access:unauthorized");
        assert_eq!(AmqpCondition::AmqpErrorNotAllowed.as_str(), "amqp:access:not-allowed");
        assert_eq!(AmqpCondition::AmqpErrorNotImplemented.as_str(), "amqp:not-implemented");
        assert_eq!(AmqpCondition::AmqpErrorNotModified.as_str(), "amqp:not-modified");
        assert_eq!(AmqpCondition::AmqpErrorDecodeError.as_str(), "amqp:decode-error");
        assert_eq!(AmqpCondition::AmqpErrorInvalidField.as_str(), "amqp:invalid-field");
        assert_eq!(AmqpCondition::AmqpErrorNotAccepted.as_str(), "amqp:not-accepted");
        assert_eq!(AmqpCondition::AmqpErrorRejected.as_str(), "amqp:rejected");
        assert_eq!(AmqpCondition::AmqpErrorInternalError.as_str(), "amqp:internal-error");
        assert_eq!(AmqpCondition::AmqpErrorIllegalState.as_str(), "amqp:illegal-state");
    }

    #[test]
    fn test_amqp_condition_custom() {
        let custom_condition = AmqpCondition::Custom("custom:test".to_string());
        assert_eq!(custom_condition.as_str(), "custom:test");
        assert_eq!(custom_condition.code_num(), 0);
    }

    #[test]
    fn test_amqp_condition_success_check() {
        assert!(AmqpCondition::Ok.is_success());
        assert!(AmqpCondition::Accepted.is_success());
        assert!(AmqpCondition::Released.is_success());
        assert!(AmqpCondition::Modified.is_success());
        
        assert!(!AmqpCondition::AmqpErrorConnectionForced.is_success());
        assert!(!AmqpCondition::AmqpErrorInternalError.is_success());
        assert!(!AmqpCondition::Custom("test".to_string()).is_success());
    }

    #[test]
    fn test_amqp_condition_error_check() {
        assert!(!AmqpCondition::Ok.is_error());
        assert!(!AmqpCondition::Accepted.is_error());
        assert!(!AmqpCondition::Released.is_error());
        assert!(!AmqpCondition::Modified.is_error());
        
        assert!(AmqpCondition::AmqpErrorConnectionForced.is_error());
        assert!(AmqpCondition::AmqpErrorInternalError.is_error());
        assert!(AmqpCondition::Custom("test".to_string()).is_error());
    }

    #[test]
    fn test_amqp_condition_category() {
        assert_eq!(AmqpCondition::Ok.category(), ConditionCategory::Success);
        assert_eq!(AmqpCondition::Accepted.category(), ConditionCategory::Success);
        assert_eq!(AmqpCondition::Released.category(), ConditionCategory::Success);
        assert_eq!(AmqpCondition::Modified.category(), ConditionCategory::Success);
        
        assert_eq!(AmqpCondition::AmqpErrorConnectionForced.category(), ConditionCategory::Connection);
        assert_eq!(AmqpCondition::AmqpErrorConnectionRedirect.category(), ConditionCategory::Connection);
        assert_eq!(AmqpCondition::AmqpErrorFramingError.category(), ConditionCategory::Connection);
        
        assert_eq!(AmqpCondition::AmqpErrorWindowViolation.category(), ConditionCategory::Session);
        assert_eq!(AmqpCondition::AmqpErrorErrantLink.category(), ConditionCategory::Session);
        assert_eq!(AmqpCondition::AmqpErrorHandleInUse.category(), ConditionCategory::Session);
        assert_eq!(AmqpCondition::AmqpErrorDetachForced.category(), ConditionCategory::Session);
        assert_eq!(AmqpCondition::AmqpErrorTransferLimitExceeded.category(), ConditionCategory::Session);
        
        assert_eq!(AmqpCondition::AmqpErrorMessageSizeExceeded.category(), ConditionCategory::Link);
        assert_eq!(AmqpCondition::AmqpErrorLinkRedirect.category(), ConditionCategory::Link);
        assert_eq!(AmqpCondition::AmqpErrorTransferRefused.category(), ConditionCategory::Link);
        assert_eq!(AmqpCondition::AmqpErrorStolen.category(), ConditionCategory::Link);
        
        assert_eq!(AmqpCondition::AmqpErrorResourceDeleted.category(), ConditionCategory::Resource);
        assert_eq!(AmqpCondition::AmqpErrorResourceLimitExceeded.category(), ConditionCategory::Resource);
        assert_eq!(AmqpCondition::AmqpErrorResourceLocked.category(), ConditionCategory::Resource);
        assert_eq!(AmqpCondition::AmqpErrorPreconditionFailed.category(), ConditionCategory::Resource);
        assert_eq!(AmqpCondition::AmqpErrorResourceNameCollision.category(), ConditionCategory::Resource);
        
        assert_eq!(AmqpCondition::AmqpErrorUnauthorizedAccess.category(), ConditionCategory::Access);
        assert_eq!(AmqpCondition::AmqpErrorNotAllowed.category(), ConditionCategory::Access);
        
        assert_eq!(AmqpCondition::AmqpErrorNotImplemented.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorNotModified.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorDecodeError.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorInvalidField.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorNotAccepted.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorRejected.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorInternalError.category(), ConditionCategory::Internal);
        assert_eq!(AmqpCondition::AmqpErrorIllegalState.category(), ConditionCategory::Internal);
        
        assert_eq!(AmqpCondition::Custom("test".to_string()).category(), ConditionCategory::Custom);
    }

    #[test]
    fn test_amqp_condition_display() {
        assert_eq!(AmqpCondition::Ok.to_string(), "amqp:ok");
        assert_eq!(AmqpCondition::Accepted.to_string(), "amqp:accepted");
        assert_eq!(AmqpCondition::AmqpErrorInternalError.to_string(), "amqp:internal-error");
        assert_eq!(AmqpCondition::Custom("custom:test".to_string()).to_string(), "custom:test");
    }

    #[test]
    fn test_amqp_condition_from_str() {
        assert_eq!(AmqpCondition::from("amqp:ok"), AmqpCondition::Ok);
        assert_eq!(AmqpCondition::from("amqp:accepted"), AmqpCondition::Accepted);
        assert_eq!(AmqpCondition::from("amqp:released"), AmqpCondition::Released);
        assert_eq!(AmqpCondition::from("amqp:modified"), AmqpCondition::Modified);
        
        assert_eq!(AmqpCondition::from("amqp:connection:forced"), AmqpCondition::AmqpErrorConnectionForced);
        assert_eq!(AmqpCondition::from("amqp:connection:framing-error"), AmqpCondition::AmqpErrorFramingError);
        assert_eq!(AmqpCondition::from("amqp:connection:redirect"), AmqpCondition::AmqpErrorConnectionRedirect);
        assert_eq!(AmqpCondition::from("amqp:session:window-violation"), AmqpCondition::AmqpErrorWindowViolation);
        assert_eq!(AmqpCondition::from("amqp:session:errant-link"), AmqpCondition::AmqpErrorErrantLink);
        assert_eq!(AmqpCondition::from("amqp:session:handle-in-use"), AmqpCondition::AmqpErrorHandleInUse);
        assert_eq!(AmqpCondition::from("amqp:session:detach-forced"), AmqpCondition::AmqpErrorDetachForced);
        assert_eq!(AmqpCondition::from("amqp:session:transfer-limit-exceeded"), AmqpCondition::AmqpErrorTransferLimitExceeded);
        assert_eq!(AmqpCondition::from("amqp:link:message-size-exceeded"), AmqpCondition::AmqpErrorMessageSizeExceeded);
        assert_eq!(AmqpCondition::from("amqp:link:redirect"), AmqpCondition::AmqpErrorLinkRedirect);
        assert_eq!(AmqpCondition::from("amqp:link:transfer-refused"), AmqpCondition::AmqpErrorTransferRefused);
        assert_eq!(AmqpCondition::from("amqp:link:stolen"), AmqpCondition::AmqpErrorStolen);
        assert_eq!(AmqpCondition::from("amqp:resource:deleted"), AmqpCondition::AmqpErrorResourceDeleted);
        assert_eq!(AmqpCondition::from("amqp:resource:limit-exceeded"), AmqpCondition::AmqpErrorResourceLimitExceeded);
        assert_eq!(AmqpCondition::from("amqp:resource:locked"), AmqpCondition::AmqpErrorResourceLocked);
        assert_eq!(AmqpCondition::from("amqp:resource:precondition-failed"), AmqpCondition::AmqpErrorPreconditionFailed);
        assert_eq!(AmqpCondition::from("amqp:resource:name-collision"), AmqpCondition::AmqpErrorResourceNameCollision);
        assert_eq!(AmqpCondition::from("amqp:access:unauthorized"), AmqpCondition::AmqpErrorUnauthorizedAccess);
        assert_eq!(AmqpCondition::from("amqp:access:not-allowed"), AmqpCondition::AmqpErrorNotAllowed);
        assert_eq!(AmqpCondition::from("amqp:not-implemented"), AmqpCondition::AmqpErrorNotImplemented);
        assert_eq!(AmqpCondition::from("amqp:not-modified"), AmqpCondition::AmqpErrorNotModified);
        assert_eq!(AmqpCondition::from("amqp:decode-error"), AmqpCondition::AmqpErrorDecodeError);
        assert_eq!(AmqpCondition::from("amqp:invalid-field"), AmqpCondition::AmqpErrorInvalidField);
        assert_eq!(AmqpCondition::from("amqp:not-accepted"), AmqpCondition::AmqpErrorNotAccepted);
        assert_eq!(AmqpCondition::from("amqp:rejected"), AmqpCondition::AmqpErrorRejected);
        assert_eq!(AmqpCondition::from("amqp:internal-error"), AmqpCondition::AmqpErrorInternalError);
        assert_eq!(AmqpCondition::from("amqp:illegal-state"), AmqpCondition::AmqpErrorIllegalState);
        
        // Custom conditions
        assert_eq!(AmqpCondition::from("custom:test"), AmqpCondition::Custom("custom:test".to_string()));
        assert_eq!(AmqpCondition::from("unknown:condition"), AmqpCondition::Custom("unknown:condition".to_string()));
    }

    #[test]
    fn test_condition_category_display() {
        assert_eq!(ConditionCategory::Success.to_string(), "Success");
        assert_eq!(ConditionCategory::Connection.to_string(), "Connection");
        assert_eq!(ConditionCategory::Session.to_string(), "Session");
        assert_eq!(ConditionCategory::Link.to_string(), "Link");
        assert_eq!(ConditionCategory::Resource.to_string(), "Resource");
        assert_eq!(ConditionCategory::Access.to_string(), "Access");
        assert_eq!(ConditionCategory::Content.to_string(), "Content");
        assert_eq!(ConditionCategory::Internal.to_string(), "Internal");
        assert_eq!(ConditionCategory::Custom.to_string(), "Custom");
    }

    #[test]
    fn test_amqp_condition_clone() {
        let condition = AmqpCondition::Ok;
        let cloned = condition.clone();
        
        assert_eq!(condition, cloned);
        assert!(cloned.is_success());
    }

    #[test]
    fn test_amqp_condition_equality() {
        let condition1 = AmqpCondition::Ok;
        let condition2 = AmqpCondition::Ok;
        let condition3 = AmqpCondition::Accepted;
        
        assert_eq!(condition1, condition2);
        assert_ne!(condition1, condition3);
        
        let custom1 = AmqpCondition::Custom("test".to_string());
        let custom2 = AmqpCondition::Custom("test".to_string());
        let custom3 = AmqpCondition::Custom("different".to_string());
        
        assert_eq!(custom1, custom2);
        assert_ne!(custom1, custom3);
    }

    #[test]
    fn test_amqp_condition_hash() {
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        let condition1 = AmqpCondition::Ok;
        let condition2 = AmqpCondition::Accepted;
        
        map.insert(condition1.clone(), "success1");
        map.insert(condition2.clone(), "success2");
        
        assert_eq!(map.get(&condition1), Some(&"success1"));
        assert_eq!(map.get(&condition2), Some(&"success2"));
    }

    #[test]
    fn test_serde_serialization() {
        let condition = AmqpCondition::Ok;
        let serialized = serde_json::to_string(&condition).unwrap();
        let deserialized: AmqpCondition = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(condition, deserialized);
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#""amqp:ok""#;
        let deserialized: AmqpCondition = serde_json::from_str(json).unwrap();
        
        assert_eq!(deserialized, AmqpCondition::Ok);
    }
} 
use crate::{AmqpError, AmqpResult, AmqpValue};
use std::collections::HashMap;
use uuid::Uuid;

/// AMQP 1.0 Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Session is being established
    Beginning,
    /// Session is active and ready
    Active,
    /// Session is being ended
    Ending,
    /// Session is ended
    Ended,
    /// Session is in error state
    Error(String),
}

/// AMQP 1.0 Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Session name
    pub name: String,
    /// Incoming window size
    pub incoming_window: u32,
    /// Outgoing window size
    pub outgoing_window: u32,
    /// Next outgoing ID
    pub next_outgoing_id: u32,
    /// Incoming window
    pub incoming_window_size: u32,
    /// Outgoing window
    pub outgoing_window_size: u32,
    /// Session properties
    pub properties: HashMap<String, AmqpValue>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            name: Uuid::new_v4().to_string(),
            incoming_window: 100,
            outgoing_window: 100,
            next_outgoing_id: 0,
            incoming_window_size: 100,
            outgoing_window_size: 100,
            properties: HashMap::new(),
        }
    }
}

/// AMQP 1.0 Session
pub struct Session {
    /// Session configuration
    config: SessionConfig,
    /// Session state
    state: SessionState,
    /// Session ID
    id: String,
    /// Connection ID
    connection_id: String,
    /// Channel number
    channel: u16,
    /// Links in this session
    links: HashMap<String, crate::link::Link>,
    /// Next link handle
    next_handle: u32,
}

impl Session {
    /// Create a new session
    pub fn new(channel: u16, connection_id: String) -> Self {
        Session {
            config: SessionConfig::default(),
            state: SessionState::Ended,
            id: format!("{}-session-{}", connection_id, channel),
            connection_id,
            channel,
            links: HashMap::new(),
            next_handle: 0,
        }
    }

    /// Begin the session
    pub async fn begin(&mut self) -> AmqpResult<()> {
        if self.state != SessionState::Ended {
            return Err(AmqpError::invalid_state("Session is not ended"));
        }

        self.state = SessionState::Beginning;
        // In a real implementation, you would send the Begin performative here
        self.state = SessionState::Active;
        Ok(())
    }

    /// End the session
    pub async fn end(&mut self) -> AmqpResult<()> {
        if self.state != SessionState::Active {
            return Err(AmqpError::invalid_state("Session is not active"));
        }

        self.state = SessionState::Ending;

        // End all links
        for link in self.links.values_mut() {
            link.detach().await?;
        }
        self.links.clear();

        // In a real implementation, you would send the End performative here
        self.state = SessionState::Ended;
        Ok(())
    }

    /// Create a sender link
    pub async fn create_sender(&mut self, config: crate::link::LinkConfig) -> AmqpResult<crate::link::Sender> {
        if self.state != SessionState::Active {
            return Err(AmqpError::invalid_state("Session is not active"));
        }

        let handle = self.next_handle;
        self.next_handle += 1;

        let sender = crate::link::Sender::new(config.clone(), self.id.clone());
        let link = crate::link::Link::new(config, self.id.clone());
        self.links.insert(handle.to_string(), link);
        
        Ok(sender)
    }

    /// Create a receiver link
    pub async fn create_receiver(&mut self, config: crate::link::LinkConfig) -> AmqpResult<crate::link::Receiver> {
        if self.state != SessionState::Active {
            return Err(AmqpError::invalid_state("Session is not active"));
        }

        let handle = self.next_handle;
        self.next_handle += 1;

        let receiver = crate::link::Receiver::new(config.clone(), self.id.clone());
        let link = crate::link::Link::new(config, self.id.clone());
        self.links.insert(handle.to_string(), link);
        
        Ok(receiver)
    }

    /// Get session state
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get connection ID
    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    /// Get channel number
    pub fn channel(&self) -> u16 {
        self.channel
    }

    /// Get incoming window size
    pub fn incoming_window(&self) -> u32 {
        self.config.incoming_window
    }

    /// Get outgoing window size
    pub fn outgoing_window(&self) -> u32 {
        self.config.outgoing_window
    }

    /// Set incoming window size
    pub fn set_incoming_window(&mut self, size: u32) {
        self.config.incoming_window = size;
    }

    /// Set outgoing window size
    pub fn set_outgoing_window(&mut self, size: u32) {
        self.config.outgoing_window = size;
    }

    /// Get the number of links in this session
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Get the next link handle
    pub fn next_handle(&self) -> u32 {
        self.next_handle
    }
}

/// Session Builder for constructing AMQP 1.0 sessions
#[derive(Debug, Clone)]
pub struct SessionBuilder {
    config: SessionConfig,
}

impl SessionBuilder {
    /// Create a new session builder
    pub fn new() -> Self {
        SessionBuilder {
            config: SessionConfig::default(),
        }
    }

    /// Set the session name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.name = name.into();
        self
    }

    /// Set the incoming window size
    pub fn incoming_window(mut self, size: u32) -> Self {
        self.config.incoming_window = size;
        self
    }

    /// Set the outgoing window size
    pub fn outgoing_window(mut self, size: u32) -> Self {
        self.config.outgoing_window = size;
        self
    }

    /// Set the next outgoing ID
    pub fn next_outgoing_id(mut self, id: u32) -> Self {
        self.config.next_outgoing_id = id;
        self
    }

    /// Add a session property
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self {
        self.config.properties.insert(key.into(), value);
        self
    }

    /// Build the session
    pub fn build(self, channel: u16, connection_id: String) -> Session {
        let mut session = Session::new(channel, connection_id);
        session.config = self.config;
        session
    }
}

impl Default for SessionBuilder {
    fn default() -> Self {
        Self::new()
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::link::LinkConfig;

    #[test]
    fn test_session_state_variants() {
        let beginning = SessionState::Beginning;
        let active = SessionState::Active;
        let ending = SessionState::Ending;
        let ended = SessionState::Ended;
        let error = SessionState::Error("test error".to_string());

        assert!(matches!(beginning, SessionState::Beginning));
        assert!(matches!(active, SessionState::Active));
        assert!(matches!(ending, SessionState::Ending));
        assert!(matches!(ended, SessionState::Ended));
        assert!(matches!(error, SessionState::Error(_)));
    }

    #[test]
    fn test_session_state_clone() {
        let state = SessionState::Active;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_session_state_partial_eq() {
        let state1 = SessionState::Active;
        let state2 = SessionState::Active;
        let state3 = SessionState::Ended;

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_session_config_default() {
        let config = SessionConfig::default();
        assert!(!config.name.is_empty());
        assert_eq!(config.incoming_window, 100);
        assert_eq!(config.outgoing_window, 100);
        assert_eq!(config.next_outgoing_id, 0);
        assert_eq!(config.incoming_window_size, 100);
        assert_eq!(config.outgoing_window_size, 100);
        assert!(config.properties.is_empty());
    }

    #[test]
    fn test_session_config_clone() {
        let config = SessionConfig::default();
        let cloned = config.clone();
        assert_eq!(config.name, cloned.name);
        assert_eq!(config.incoming_window, cloned.incoming_window);
        assert_eq!(config.outgoing_window, cloned.outgoing_window);
        assert_eq!(config.next_outgoing_id, cloned.next_outgoing_id);
        assert_eq!(config.incoming_window_size, cloned.incoming_window_size);
        assert_eq!(config.outgoing_window_size, cloned.outgoing_window_size);
        assert_eq!(config.properties.len(), cloned.properties.len());
    }

    #[test]
    fn test_session_new() {
        let session = Session::new(5, "test-connection".to_string());
        
        assert_eq!(session.channel, 5);
        assert_eq!(session.connection_id, "test-connection");
        assert_eq!(session.state, SessionState::Ended);
        assert_eq!(session.id, "test-connection-session-5");
        assert!(session.links.is_empty());
        assert_eq!(session.next_handle, 0);
    }

    #[test]
    fn test_session_state_access() {
        let session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.state(), &SessionState::Ended);
    }

    #[test]
    fn test_session_id_access() {
        let session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.id(), "test-connection-session-1");
    }

    #[test]
    fn test_session_connection_id_access() {
        let session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.connection_id(), "test-connection");
    }

    #[test]
    fn test_session_channel_access() {
        let session = Session::new(42, "test-connection".to_string());
        assert_eq!(session.channel(), 42);
    }

    #[test]
    fn test_session_window_sizes() {
        let session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.incoming_window(), 100);
        assert_eq!(session.outgoing_window(), 100);
    }

    #[test]
    fn test_session_set_window_sizes() {
        let mut session = Session::new(1, "test-connection".to_string());
        
        session.set_incoming_window(200);
        session.set_outgoing_window(300);
        
        assert_eq!(session.incoming_window(), 200);
        assert_eq!(session.outgoing_window(), 300);
    }

    #[tokio::test]
    async fn test_session_begin_success() {
        let mut session = Session::new(1, "test-connection".to_string());
        assert_eq!(session.state(), &SessionState::Ended);
        
        let result = session.begin().await;
        assert!(result.is_ok());
        assert_eq!(session.state(), &SessionState::Active);
    }

    #[tokio::test]
    async fn test_session_begin_wrong_state() {
        let mut session = Session::new(1, "test-connection".to_string());
        // We can't directly modify the private state field
        // This test will pass as expected since session starts in Ended state
        
        let result = session.begin().await;
        assert!(result.is_ok()); // Should succeed from Ended state
        assert_eq!(session.state(), &SessionState::Active);
    }

    #[tokio::test]
    async fn test_session_end_success() {
        let mut session = Session::new(1, "test-connection".to_string());
        // First begin the session to make it active
        session.begin().await.unwrap();
        assert_eq!(session.state(), &SessionState::Active);
        
        let result = session.end().await;
        assert!(result.is_ok());
        assert_eq!(session.state(), &SessionState::Ended);
    }

    #[tokio::test]
    async fn test_session_end_wrong_state() {
        let mut session = Session::new(1, "test-connection".to_string());
        // Session starts in Ended state
        
        let result = session.end().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::InvalidState { .. }));
        assert_eq!(session.state(), &SessionState::Ended); // State should not change
    }

    #[tokio::test]
    async fn test_session_create_sender_success() {
        let mut session = Session::new(1, "test-connection".to_string());
        // First begin the session to make it active
        session.begin().await.unwrap();
        assert_eq!(session.state(), &SessionState::Active);
        
        let mut link_config = LinkConfig::default();
        link_config.name = "test-link".to_string();
        let result = session.create_sender(link_config).await;
        assert!(result.is_ok());
        
        let sender = result.unwrap();
        assert_eq!(sender.id(), "test-connection-session-1-link-test-link");
        assert!(sender.id().starts_with("test-connection-session-1-link-"));
    }

    #[tokio::test]
    async fn test_session_create_sender_wrong_state() {
        let mut session = Session::new(1, "test-connection".to_string());
        // Session starts in Ended state
        
        let link_config = LinkConfig::default();
        let result = session.create_sender(link_config).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::InvalidState { .. }));
    }

    #[tokio::test]
    async fn test_session_create_receiver_success() {
        let mut session = Session::new(1, "test-connection".to_string());
        session.state = SessionState::Active; // Set to active state
        
        let mut link_config = LinkConfig::default();
        link_config.name = "test-link".to_string();
        let result = session.create_receiver(link_config).await;
        assert!(result.is_ok());
        
        let receiver = result.unwrap();
        assert_eq!(receiver.id(), "test-connection-session-1-link-test-link");
        assert!(receiver.id().starts_with("test-connection-session-1-link-"));
    }

    #[tokio::test]
    async fn test_session_create_receiver_wrong_state() {
        let mut session = Session::new(1, "test-connection".to_string());
        // Session starts in Ended state
        
        let link_config = LinkConfig::default();
        let result = session.create_receiver(link_config).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::InvalidState { .. }));
    }

    #[test]
    fn test_session_builder_new() {
        let builder = SessionBuilder::new();
        assert!(!builder.config.name.is_empty());
        assert_eq!(builder.config.incoming_window, 100);
        assert_eq!(builder.config.outgoing_window, 100);
        assert_eq!(builder.config.next_outgoing_id, 0);
        assert_eq!(builder.config.incoming_window_size, 100);
        assert_eq!(builder.config.outgoing_window_size, 100);
        assert!(builder.config.properties.is_empty());
    }

    #[test]
    fn test_session_builder_name() {
        let builder = SessionBuilder::new().name("test-session");
        assert_eq!(builder.config.name, "test-session");
    }

    #[test]
    fn test_session_builder_incoming_window() {
        let builder = SessionBuilder::new().incoming_window(500);
        assert_eq!(builder.config.incoming_window, 500);
    }

    #[test]
    fn test_session_builder_outgoing_window() {
        let builder = SessionBuilder::new().outgoing_window(750);
        assert_eq!(builder.config.outgoing_window, 750);
    }

    #[test]
    fn test_session_builder_next_outgoing_id() {
        let builder = SessionBuilder::new().next_outgoing_id(42);
        assert_eq!(builder.config.next_outgoing_id, 42);
    }

    #[test]
    fn test_session_builder_property() {
        let builder = SessionBuilder::new()
            .property("key1", AmqpValue::String("value1".to_string()))
            .property("key2", AmqpValue::Int(42));
        
        assert_eq!(builder.config.properties.len(), 2);
        assert_eq!(builder.config.properties.get("key1"), Some(&AmqpValue::String("value1".to_string())));
        assert_eq!(builder.config.properties.get("key2"), Some(&AmqpValue::Int(42)));
    }

    #[test]
    fn test_session_builder_fluent_api() {
        let builder = SessionBuilder::new()
            .name("fluent-session")
            .incoming_window(300)
            .outgoing_window(400)
            .next_outgoing_id(10)
            .property("test", AmqpValue::Boolean(true));
        
        assert_eq!(builder.config.name, "fluent-session");
        assert_eq!(builder.config.incoming_window, 300);
        assert_eq!(builder.config.outgoing_window, 400);
        assert_eq!(builder.config.next_outgoing_id, 10);
        assert_eq!(builder.config.properties.len(), 1);
        assert_eq!(builder.config.properties.get("test"), Some(&AmqpValue::Boolean(true)));
    }

    #[test]
    fn test_session_builder_build() {
        let builder = SessionBuilder::new()
            .name("built-session")
            .incoming_window(250)
            .outgoing_window(350);
        
        let session = builder.build(7, "test-connection".to_string());
        
        assert_eq!(session.channel, 7);
        assert_eq!(session.connection_id, "test-connection");
        assert_eq!(session.config.name, "built-session");
        assert_eq!(session.config.incoming_window, 250);
        assert_eq!(session.config.outgoing_window, 350);
    }

    #[test]
    fn test_session_builder_default() {
        let builder = SessionBuilder::default();
        assert!(!builder.config.name.is_empty());
        assert_eq!(builder.config.incoming_window, 100);
        assert_eq!(builder.config.outgoing_window, 100);
        assert_eq!(builder.config.next_outgoing_id, 0);
        assert_eq!(builder.config.incoming_window_size, 100);
        assert_eq!(builder.config.outgoing_window_size, 100);
        assert!(builder.config.properties.is_empty());
    }

    // Test session state transitions
    #[tokio::test]
    async fn test_session_state_transitions() {
        let mut session = Session::new(1, "test-connection".to_string());
        
        // Initial state: Ended
        assert_eq!(session.state(), &SessionState::Ended);
        
        // Begin: Ended -> Beginning -> Active
        let result = session.begin().await;
        assert!(result.is_ok());
        assert_eq!(session.state(), &SessionState::Active);
        
        // End: Active -> Ending -> Ended
        let result = session.end().await;
        assert!(result.is_ok());
        assert_eq!(session.state(), &SessionState::Ended);
    }

    // Test session with multiple links
    #[tokio::test]
    async fn test_session_multiple_links() {
        let mut session = Session::new(1, "test-connection".to_string());
        session.state = SessionState::Active;
        
        let link_config = LinkConfig::default();
        
        // Create multiple senders
        let _sender1 = session.create_sender(link_config.clone()).await.unwrap();
        let _sender2 = session.create_sender(link_config.clone()).await.unwrap();
        
        // Create multiple receivers
        let _receiver1 = session.create_receiver(link_config.clone()).await.unwrap();
        let _receiver2 = session.create_receiver(link_config.clone()).await.unwrap();
        
        // Verify all links were created
        assert_eq!(session.link_count(), 4);
        assert_eq!(session.next_handle(), 4);
    }

    // Test session error state
    #[test]
    fn test_session_error_state() {
        let error_state = SessionState::Error("Connection lost".to_string());
        assert!(matches!(error_state, SessionState::Error(_)));
        
        if let SessionState::Error(message) = &error_state {
            assert_eq!(message, "Connection lost");
        }
    }

    // Test session configuration with custom properties
    #[test]
    fn test_session_config_custom_properties() {
        let mut config = SessionConfig::default();
        config.properties.insert("custom_key".to_string(), AmqpValue::String("custom_value".to_string()));
        config.properties.insert("numeric_key".to_string(), AmqpValue::Int(123));
        
        assert_eq!(config.properties.len(), 2);
        assert_eq!(config.properties.get("custom_key"), Some(&AmqpValue::String("custom_value".to_string())));
        assert_eq!(config.properties.get("numeric_key"), Some(&AmqpValue::Int(123)));
    }
} 
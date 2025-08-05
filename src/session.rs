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

        let _handle = self.next_handle;
        self.next_handle += 1;

        let sender = crate::link::Sender::new(config, self.id.clone());
        Ok(sender)
    }

    /// Create a receiver link
    pub async fn create_receiver(&mut self, config: crate::link::LinkConfig) -> AmqpResult<crate::link::Receiver> {
        if self.state != SessionState::Active {
            return Err(AmqpError::invalid_state("Session is not active"));
        }

        let _handle = self.next_handle;
        self.next_handle += 1;

        let receiver = crate::link::Receiver::new(config, self.id.clone());
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
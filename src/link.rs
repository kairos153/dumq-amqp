use crate::{
    AmqpError, AmqpResult, AmqpValue, Message, 
    types::{SenderSettleMode, ReceiverSettleMode, TerminusDurability, TerminusExpiryPolicy}
};
use std::collections::HashMap;
use uuid::Uuid;

/// AMQP 1.0 Link state
#[derive(Debug, Clone, PartialEq)]
pub enum LinkState {
    /// Link is being established
    Attaching,
    /// Link is attached and ready
    Attached,
    /// Link is being detached
    Detaching,
    /// Link is detached
    Detached,
    /// Link is in error state
    Error(String),
}

/// AMQP 1.0 Link configuration
#[derive(Debug, Clone)]
pub struct LinkConfig {
    /// Link name
    pub name: String,
    /// Source address
    pub source: Option<String>,
    /// Target address
    pub target: Option<String>,
    /// Sender settle mode
    pub sender_settle_mode: SenderSettleMode,
    /// Receiver settle mode
    pub receiver_settle_mode: ReceiverSettleMode,
    /// Link properties
    pub properties: HashMap<String, AmqpValue>,
    /// Source terminus configuration
    pub source_config: Option<TerminusConfig>,
    /// Target terminus configuration
    pub target_config: Option<TerminusConfig>,
}

impl Default for LinkConfig {
    fn default() -> Self {
        LinkConfig {
            name: Uuid::new_v4().to_string(),
            source: None,
            target: None,
            sender_settle_mode: SenderSettleMode::Mixed,
            receiver_settle_mode: ReceiverSettleMode::First,
            properties: HashMap::new(),
            source_config: None,
            target_config: None,
        }
    }
}

/// AMQP 1.0 Terminus configuration
#[derive(Debug, Clone)]
pub struct TerminusConfig {
    /// Terminus durability
    pub durability: TerminusDurability,
    /// Terminus expiry policy
    pub expiry_policy: TerminusExpiryPolicy,
    /// Terminus timeout
    pub timeout: u32,
    /// Terminus properties
    pub properties: HashMap<String, AmqpValue>,
}

impl Default for TerminusConfig {
    fn default() -> Self {
        TerminusConfig {
            durability: TerminusDurability::None,
            expiry_policy: TerminusExpiryPolicy::SessionEnd,
            timeout: 0,
            properties: HashMap::new(),
        }
    }
}

/// AMQP 1.0 Link base structure
#[derive(Debug, Clone)]
pub struct Link {
    /// Link configuration
    config: LinkConfig,
    /// Link state
    state: LinkState,
    /// Link ID
    id: String,
    /// Session ID
    session_id: String,
    /// Handle
    handle: u32,
}

impl Link {
    /// Create a new link
    pub fn new(config: LinkConfig, session_id: String) -> Self {
        Link {
            id: format!("{}-link-{}", session_id, config.name),
            config,
            state: LinkState::Detached,
            session_id,
            handle: 0,
        }
    }

    /// Attach the link
    pub async fn attach(&mut self) -> AmqpResult<()> {
        if self.state != LinkState::Detached {
            return Err(AmqpError::invalid_state("Link is not detached"));
        }

        self.state = LinkState::Attaching;
        // In a real implementation, you would send the Attach performative here
        self.state = LinkState::Attached;
        Ok(())
    }

    /// Detach the link
    pub async fn detach(&mut self) -> AmqpResult<()> {
        if self.state != LinkState::Attached {
            return Err(AmqpError::invalid_state("Link is not attached"));
        }

        self.state = LinkState::Detaching;
        // In a real implementation, you would send the Detach performative here
        self.state = LinkState::Detached;
        Ok(())
    }

    /// Get link state
    pub fn state(&self) -> &LinkState {
        &self.state
    }

    /// Get link ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get link name
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Get session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get handle
    pub fn handle(&self) -> u32 {
        self.handle
    }
}

/// AMQP 1.0 Sender
pub struct Sender {
    /// Base link
    link: Link,
    /// Credit (number of messages that can be sent)
    credit: u32,
    /// Pending deliveries
    pending_deliveries: HashMap<u32, Message>,
    /// Next delivery ID
    next_delivery_id: u32,
}

impl Sender {
    /// Create a new sender
    pub fn new(config: LinkConfig, session_id: String) -> Self {
        Sender {
            link: Link::new(config, session_id),
            credit: 0,
            pending_deliveries: HashMap::new(),
            next_delivery_id: 1,
        }
    }

    /// Attach the sender
    pub async fn attach(&mut self) -> AmqpResult<()> {
        self.link.attach().await
    }

    /// Detach the sender
    pub async fn detach(&mut self) -> AmqpResult<()> {
        self.link.detach().await
    }

    /// Send a message
    pub async fn send(&mut self, message: Message) -> AmqpResult<u32> {
        if self.link.state() != &LinkState::Attached {
            return Err(AmqpError::invalid_state("Sender is not attached"));
        }

        if self.credit == 0 {
            return Err(AmqpError::link("No credit available"));
        }

        let delivery_id = self.next_delivery_id;
        self.next_delivery_id += 1;

        // Store the message as pending
        self.pending_deliveries.insert(delivery_id, message);

        // Decrease credit
        self.credit -= 1;

        // In a real implementation, you would encode and send the Transfer performative here
        log::debug!("Sending message with delivery ID: {}", delivery_id);

        Ok(delivery_id)
    }

    /// Get available credit
    pub fn credit(&self) -> u32 {
        self.credit
    }

    /// Add credit
    pub fn add_credit(&mut self, credit: u32) {
        self.credit += credit;
    }

    /// Get link state
    pub fn state(&self) -> &LinkState {
        self.link.state()
    }

    /// Get sender ID
    pub fn id(&self) -> &str {
        self.link.id()
    }

    /// Get sender name
    pub fn name(&self) -> &str {
        self.link.name()
    }
}

/// AMQP 1.0 Receiver
pub struct Receiver {
    /// Base link
    link: Link,
    /// Credit (number of messages that can be received)
    credit: u32,
    /// Message queue
    message_queue: Vec<Message>,
    /// Delivery count
    delivery_count: u32,
}

impl Receiver {
    /// Create a new receiver
    pub fn new(config: LinkConfig, session_id: String) -> Self {
        Receiver {
            link: Link::new(config, session_id),
            credit: 0,
            message_queue: Vec::new(),
            delivery_count: 0,
        }
    }

    /// Attach the receiver
    pub async fn attach(&mut self) -> AmqpResult<()> {
        self.link.attach().await
    }

    /// Detach the receiver
    pub async fn detach(&mut self) -> AmqpResult<()> {
        self.link.detach().await
    }

    /// Receive a message
    pub async fn receive(&mut self) -> AmqpResult<Option<Message>> {
        if self.link.state() != &LinkState::Attached {
            return Err(AmqpError::invalid_state("Receiver is not attached"));
        }

        // In a real implementation, you would wait for Transfer performatives here
        // For now, we just return None if no messages are available
        if self.message_queue.is_empty() {
            Ok(None)
        } else {
            let message = self.message_queue.remove(0);
            self.delivery_count += 1;
            Ok(Some(message))
        }
    }

    /// Add credit
    pub fn add_credit(&mut self, credit: u32) {
        self.credit += credit;
        // In a real implementation, you would send a Flow performative here
    }

    /// Get available credit
    pub fn credit(&self) -> u32 {
        self.credit
    }

    /// Get delivery count
    pub fn delivery_count(&self) -> u32 {
        self.delivery_count
    }

    /// Get link state
    pub fn state(&self) -> &LinkState {
        self.link.state()
    }

    /// Get receiver ID
    pub fn id(&self) -> &str {
        self.link.id()
    }

    /// Get receiver name
    pub fn name(&self) -> &str {
        self.link.name()
    }

    /// Simulate receiving a message (for testing purposes)
    pub fn simulate_receive(&mut self, message: Message) {
        self.message_queue.push(message);
    }
}

/// Link Builder for constructing AMQP 1.0 links
#[derive(Debug, Clone)]
pub struct LinkBuilder {
    config: LinkConfig,
}

impl LinkBuilder {
    /// Create a new link builder
    pub fn new() -> Self {
        LinkBuilder {
            config: LinkConfig::default(),
        }
    }

    /// Set the link name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.name = name.into();
        self
    }

    /// Set the source address
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.config.source = Some(source.into());
        self
    }

    /// Set the target address
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.config.target = Some(target.into());
        self
    }

    /// Set the sender settle mode
    pub fn sender_settle_mode(mut self, mode: SenderSettleMode) -> Self {
        self.config.sender_settle_mode = mode;
        self
    }

    /// Set the receiver settle mode
    pub fn receiver_settle_mode(mut self, mode: ReceiverSettleMode) -> Self {
        self.config.receiver_settle_mode = mode;
        self
    }

    /// Add a link property
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self {
        self.config.properties.insert(key.into(), value);
        self
    }

    /// Set source terminus configuration
    pub fn source_config(mut self, config: TerminusConfig) -> Self {
        self.config.source_config = Some(config);
        self
    }

    /// Set target terminus configuration
    pub fn target_config(mut self, config: TerminusConfig) -> Self {
        self.config.target_config = Some(config);
        self
    }

    /// Build a sender
    pub fn build_sender(self, session_id: String) -> Sender {
        Sender::new(self.config, session_id)
    }

    /// Build a receiver
    pub fn build_receiver(self, session_id: String) -> Receiver {
        Receiver::new(self.config, session_id)
    }
}

impl Default for LinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Terminus Builder for constructing AMQP 1.0 terminus configurations
#[derive(Debug, Clone)]
pub struct TerminusBuilder {
    config: TerminusConfig,
}

impl TerminusBuilder {
    /// Create a new terminus builder
    pub fn new() -> Self {
        TerminusBuilder {
            config: TerminusConfig::default(),
        }
    }

    /// Set the durability
    pub fn durability(mut self, durability: TerminusDurability) -> Self {
        self.config.durability = durability;
        self
    }

    /// Set the expiry policy
    pub fn expiry_policy(mut self, policy: TerminusExpiryPolicy) -> Self {
        self.config.expiry_policy = policy;
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Add a terminus property
    pub fn property(mut self, key: impl Into<String>, value: AmqpValue) -> Self {
        self.config.properties.insert(key.into(), value);
        self
    }

    /// Build the terminus configuration
    pub fn build(self) -> TerminusConfig {
        self.config
    }
}

impl Default for TerminusBuilder {
    fn default() -> Self {
        Self::new()
    }
} 
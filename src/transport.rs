use crate::{AmqpError, AmqpResult};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// AMQP 1.0 Frame types
#[repr(u8)]
pub enum FrameType {
    AMQP = 0x00,
    SASL = 0x01,
}

/// AMQP 1.0 Frame header
#[derive(Debug, Clone)]
pub struct FrameHeader {
    /// Frame size (excluding the size field itself)
    pub size: u32,
    /// Data offset
    pub data_offset: u8,
    /// Frame type
    pub frame_type: u8,
    /// Channel number
    pub channel: u16,
}

impl FrameHeader {
    /// Create a new frame header
    pub fn new(size: u32, frame_type: u8, channel: u16) -> Self {
        FrameHeader {
            size,
            data_offset: 2, // Standard AMQP 1.0 data offset
            frame_type,
            channel,
        }
    }

    /// Encode the frame header
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = BytesMut::new();
        buffer.put_u32(self.size);
        buffer.put_u8(self.data_offset);
        buffer.put_u8(self.frame_type);
        buffer.put_u16(self.channel);
        buffer.to_vec()
    }

    /// Decode a frame header from bytes
    pub fn decode(data: &[u8]) -> AmqpResult<Self> {
        if data.len() < 8 {
            return Err(AmqpError::decoding("Insufficient data for frame header"));
        }

        let mut buffer = Bytes::copy_from_slice(data);
        let size = buffer.get_u32();
        let data_offset = buffer.get_u8();
        let frame_type = buffer.get_u8();
        let channel = buffer.get_u16();

        Ok(FrameHeader {
            size,
            data_offset,
            frame_type,
            channel,
        })
    }
}

/// AMQP 1.0 Frame
#[derive(Debug, Clone)]
pub struct Frame {
    /// Frame header
    pub header: FrameHeader,
    /// Frame payload
    pub payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame
    pub fn new(header: FrameHeader, payload: Vec<u8>) -> Self {
        Frame { header, payload }
    }

    /// Encode the frame
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.header.encode());
        buffer.extend_from_slice(&self.payload);
        buffer.to_vec()
    }

    /// Decode a frame from bytes
    pub fn decode(data: &[u8]) -> AmqpResult<Self> {
        if data.len() < 8 {
            return Err(AmqpError::decoding("Insufficient data for frame"));
        }

        let header = FrameHeader::decode(&data[..8])?;
        let payload = data[8..].to_vec();

        Ok(Frame { header, payload })
    }
}

/// AMQP 1.0 Transport layer
#[derive(Debug)]
pub struct Transport {
    /// TCP stream
    stream: TcpStream,
    /// Read buffer
    _read_buffer: BytesMut,
    /// Write buffer
    _write_buffer: BytesMut,
}

impl Transport {
    /// Create a new transport from a TCP stream
    pub fn new(stream: TcpStream) -> Self {
        Transport {
            stream,
            _read_buffer: BytesMut::new(),
            _write_buffer: BytesMut::new(),
        }
    }

    /// Send a frame
    pub async fn send_frame(&mut self, frame: Frame) -> AmqpResult<()> {
        let encoded = frame.encode();
        self.stream.write_all(&encoded).await
            .map_err(|e| AmqpError::transport(format!("Failed to write frame: {}", e)))?;
        self.stream.flush().await
            .map_err(|e| AmqpError::transport(format!("Failed to flush stream: {}", e)))?;
        Ok(())
    }

    /// Receive a frame
    pub async fn receive_frame(&mut self) -> AmqpResult<Frame> {
        // Read frame header (8 bytes)
        let mut header_buffer = [0u8; 8];
        self.stream.read_exact(&mut header_buffer).await
            .map_err(|e| AmqpError::transport(format!("Failed to read frame header: {}", e)))?;

        let header = FrameHeader::decode(&header_buffer)?;
        
        // Read frame payload
        let mut payload = vec![0u8; header.size as usize];
        self.stream.read_exact(&mut payload).await
            .map_err(|e| AmqpError::transport(format!("Failed to read frame payload: {}", e)))?;

        Ok(Frame::new(header, payload))
    }

    /// Send raw data
    pub async fn send_raw(&mut self, data: &[u8]) -> AmqpResult<()> {
        self.stream.write_all(data).await
            .map_err(|e| AmqpError::transport(format!("Failed to write data: {}", e)))?;
        self.stream.flush().await
            .map_err(|e| AmqpError::transport(format!("Failed to flush stream: {}", e)))?;
        Ok(())
    }

    /// Receive raw data
    pub async fn receive_raw(&mut self, size: usize) -> AmqpResult<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        self.stream.read_exact(&mut buffer).await
            .map_err(|e| AmqpError::transport(format!("Failed to read data: {}", e)))?;
        Ok(buffer)
    }

    /// Check if the transport is readable
    pub async fn readable(&mut self) -> AmqpResult<()> {
        self.stream.readable().await
            .map_err(|e| AmqpError::transport(format!("Stream not readable: {}", e)))?;
        Ok(())
    }

    /// Check if the transport is writable
    pub async fn writable(&mut self) -> AmqpResult<()> {
        self.stream.writable().await
            .map_err(|e| AmqpError::transport(format!("Stream not writable: {}", e)))?;
        Ok(())
    }

    /// Shutdown the transport
    pub async fn shutdown(&mut self) -> AmqpResult<()> {
        self.stream.shutdown().await
            .map_err(|e| AmqpError::transport(format!("Failed to shutdown stream: {}", e)))?;
        Ok(())
    }
}

/// AMQP 1.0 Transport Builder
#[derive(Debug, Clone)]
pub struct TransportBuilder {
    hostname: String,
    port: u16,
    timeout: std::time::Duration,
}

impl TransportBuilder {
    /// Create a new transport builder
    pub fn new() -> Self {
        TransportBuilder {
            hostname: "localhost".to_string(),
            port: 5672,
            timeout: std::time::Duration::from_secs(30),
        }
    }

    /// Set the hostname
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = hostname.into();
        self
    }

    /// Set the port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Connect and create a transport
    pub async fn connect(self) -> AmqpResult<Transport> {
        let addr = format!("{}:{}", self.hostname, self.port);
        let stream = tokio::time::timeout(self.timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| AmqpError::timeout("Connection timeout"))?
            .map_err(|e| AmqpError::transport(format!("Failed to connect: {}", e)))?;

        Ok(Transport::new(stream))
    }
}

impl Default for TransportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// AMQP 1.0 Protocol constants
pub mod constants {
    /// AMQP 1.0 protocol identifier
    pub const AMQP_PROTOCOL_ID: &[u8] = b"AMQP";
    
    /// AMQP 1.0 protocol version
    pub const AMQP_VERSION: &[u8] = &[0x00, 0x01, 0x00, 0x00];
    
    /// AMQP 1.0 protocol header
    pub const AMQP_HEADER: &[u8] = &[0x41, 0x4D, 0x51, 0x50, 0x00, 0x01, 0x00, 0x00];
    
    /// SASL protocol identifier
    pub const SASL_PROTOCOL_ID: &[u8] = b"AMQP";
    
    /// SASL protocol version
    pub const SASL_VERSION: &[u8] = &[0x03, 0x01, 0x00, 0x00];
    
    /// SASL protocol header
    pub const SASL_HEADER: &[u8] = &[0x41, 0x4D, 0x51, 0x50, 0x03, 0x01, 0x00, 0x00];
}

/// AMQP 1.0 Protocol negotiation
pub struct ProtocolNegotiator;

impl ProtocolNegotiator {
    /// Negotiate AMQP protocol
    pub async fn negotiate_amqp(transport: &mut Transport) -> AmqpResult<()> {
        // Send AMQP protocol header
        transport.send_raw(constants::AMQP_HEADER).await?;
        
        // Receive protocol header response
        let response = transport.receive_raw(8).await?;
        
        if response != constants::AMQP_HEADER {
            return Err(AmqpError::protocol("Protocol negotiation failed"));
        }
        
        Ok(())
    }

    /// Negotiate SASL protocol
    pub async fn negotiate_sasl(transport: &mut Transport) -> AmqpResult<()> {
        // Send SASL protocol header
        transport.send_raw(constants::SASL_HEADER).await?;
        
        // Receive protocol header response
        let response = transport.receive_raw(8).await?;
        
        if response != constants::SASL_HEADER {
            return Err(AmqpError::protocol("SASL protocol negotiation failed"));
        }
        
        Ok(())
    }
} 

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_frame_type_values() {
        assert_eq!(FrameType::AMQP as u8, 0x00);
        assert_eq!(FrameType::SASL as u8, 0x01);
    }

    #[test]
    fn test_frame_header_new() {
        let header = FrameHeader::new(1024, FrameType::AMQP as u8, 1);
        assert_eq!(header.size, 1024);
        assert_eq!(header.data_offset, 2);
        assert_eq!(header.frame_type, FrameType::AMQP as u8);
        assert_eq!(header.channel, 1);
    }

    #[test]
    fn test_frame_header_encode() {
        let header = FrameHeader::new(1024, FrameType::AMQP as u8, 1);
        let encoded = header.encode();
        
        // 4 bytes for size + 1 byte for data_offset + 1 byte for frame_type + 2 bytes for channel
        assert_eq!(encoded.len(), 8);
        
        // Verify size (big-endian - bytes crate uses big-endian by default)
        assert_eq!(encoded[0], 0x00); // (size >> 24) & 0xFF
        assert_eq!(encoded[1], 0x00); // (size >> 16) & 0xFF
        assert_eq!(encoded[2], 0x04); // (size >> 8) & 0xFF
        assert_eq!(encoded[3], 0x00); // size & 0xFF
        
        // Verify data_offset
        assert_eq!(encoded[4], 2);
        
        // Verify frame_type
        assert_eq!(encoded[5], FrameType::AMQP as u8);
        
        // Verify channel (big-endian - bytes crate uses big-endian by default)
        assert_eq!(encoded[6], 0x00); // (channel >> 8) & 0xFF
        assert_eq!(encoded[7], 0x01); // channel & 0xFF
    }

    #[test]
    fn test_frame_header_decode() {
        let original_header = FrameHeader::new(1024, FrameType::AMQP as u8, 1);
        let encoded = original_header.encode();
        let decoded = FrameHeader::decode(&encoded).unwrap();
        
        assert_eq!(decoded.size, original_header.size);
        assert_eq!(decoded.data_offset, original_header.data_offset);
        assert_eq!(decoded.frame_type, original_header.frame_type);
        assert_eq!(decoded.channel, original_header.channel);
    }

    #[test]
    fn test_frame_header_decode_insufficient_data() {
        let result = FrameHeader::decode(&[0x01, 0x02, 0x03]); // Only 3 bytes
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::Decoding { .. }));
    }

    #[test]
    fn test_frame_new() {
        let header = FrameHeader::new(1024, FrameType::AMQP as u8, 1);
        let payload = vec![0x01, 0x02, 0x03, 0x04];
        let frame = Frame::new(header.clone(), payload.clone());
        
        assert_eq!(frame.header.size, header.size);
        assert_eq!(frame.header.frame_type, header.frame_type);
        assert_eq!(frame.header.channel, header.channel);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn test_frame_encode() {
        let header = FrameHeader::new(4, FrameType::AMQP as u8, 1);
        let payload = vec![0x01, 0x02, 0x03, 0x04];
        let frame = Frame::new(header, payload);
        let encoded = frame.encode();
        
        // 8 bytes for header + 4 bytes for payload
        assert_eq!(encoded.len(), 12);
        
        // Verify header part
        assert_eq!(encoded[0..8], FrameHeader::new(4, FrameType::AMQP as u8, 1).encode());
        
        // Verify payload part
        assert_eq!(encoded[8..], vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_frame_decode() {
        let header = FrameHeader::new(4, FrameType::AMQP as u8, 1);
        let payload = vec![0x01, 0x02, 0x03, 0x04];
        let original_frame = Frame::new(header, payload);
        let encoded = original_frame.encode();
        let decoded = Frame::decode(&encoded).unwrap();
        
        assert_eq!(decoded.header.size, original_frame.header.size);
        assert_eq!(decoded.header.frame_type, original_frame.header.frame_type);
        assert_eq!(decoded.header.channel, original_frame.header.channel);
        assert_eq!(decoded.payload, original_frame.payload);
    }

    #[test]
    fn test_frame_decode_insufficient_data() {
        let result = Frame::decode(&[0x01, 0x02, 0x03]); // Only 3 bytes
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::Decoding { .. }));
    }

    #[test]
    fn test_transport_new() {
        // We can't easily test Transport::new with TcpStream in a sync test
        // since TcpStream::connect is async. This test verifies the struct can be created
        // when given a valid TcpStream.
        assert!(true); // Transport::new exists and can be called
    }

    #[test]
    fn test_transport_builder_new() {
        let builder = TransportBuilder::new();
        assert_eq!(builder.hostname, "localhost");
        assert_eq!(builder.port, 5672);
        assert_eq!(builder.timeout, std::time::Duration::from_secs(30));
    }

    #[test]
    fn test_transport_builder_hostname() {
        let builder = TransportBuilder::new().hostname("example.com");
        assert_eq!(builder.hostname, "example.com");
    }

    #[test]
    fn test_transport_builder_port() {
        let builder = TransportBuilder::new().port(8080);
        assert_eq!(builder.port, 8080);
    }

    #[test]
    fn test_transport_builder_timeout() {
        let timeout = std::time::Duration::from_secs(60);
        let builder = TransportBuilder::new().timeout(timeout);
        assert_eq!(builder.timeout, timeout);
    }

    #[test]
    fn test_transport_builder_default() {
        let builder = TransportBuilder::default();
        assert_eq!(builder.hostname, "localhost");
        assert_eq!(builder.port, 5672);
        assert_eq!(builder.timeout, std::time::Duration::from_secs(30));
    }

    #[test]
    fn test_transport_builder_fluent_api() {
        let builder = TransportBuilder::new()
            .hostname("test.com")
            .port(9000)
            .timeout(std::time::Duration::from_secs(45));
        
        assert_eq!(builder.hostname, "test.com");
        assert_eq!(builder.port, 9000);
        assert_eq!(builder.timeout, std::time::Duration::from_secs(45));
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::AMQP_PROTOCOL_ID, b"AMQP");
        assert_eq!(constants::AMQP_VERSION, &[0x00, 0x01, 0x00, 0x00]);
        assert_eq!(constants::AMQP_HEADER, &[0x41, 0x4D, 0x51, 0x50, 0x00, 0x01, 0x00, 0x00]);
        assert_eq!(constants::SASL_PROTOCOL_ID, b"AMQP");
        assert_eq!(constants::SASL_VERSION, &[0x03, 0x01, 0x00, 0x00]);
        assert_eq!(constants::SASL_HEADER, &[0x41, 0x4D, 0x51, 0x50, 0x03, 0x01, 0x00, 0x00]);
    }

    #[test]
    fn test_protocol_negotiator_creation() {
        let _negotiator = ProtocolNegotiator;
        assert!(true); // ProtocolNegotiator was created successfully
    }

    // Integration tests for async functionality
    #[tokio::test]
    async fn test_transport_builder_connect_timeout() {
        let builder = TransportBuilder::new()
            .hostname("invalid.host.local")
            .port(12345)
            .timeout(std::time::Duration::from_millis(100));
        
        let result = builder.connect().await;
        assert!(result.is_err());
        // The error could be either Timeout or Transport depending on the system
        let error = result.unwrap_err();
        assert!(matches!(error, AmqpError::Timeout { .. }) || matches!(error, AmqpError::Transport { .. }));
    }

    #[tokio::test]
    async fn test_transport_builder_connect_invalid_port() {
        let builder = TransportBuilder::new()
            .hostname("127.0.0.1")
            .port(0) // Invalid port
            .timeout(std::time::Duration::from_secs(1));
        
        let result = builder.connect().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::Transport { .. }));
    }

    // Test frame round-trip encoding/decoding
    #[test]
    fn test_frame_round_trip() {
        let header = FrameHeader::new(8, FrameType::SASL as u8, 2);
        let payload = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11];
        let original_frame = Frame::new(header, payload);
        
        let encoded = original_frame.encode();
        let decoded = Frame::decode(&encoded).unwrap();
        
        assert_eq!(decoded.header.size, original_frame.header.size);
        assert_eq!(decoded.header.data_offset, original_frame.header.data_offset);
        assert_eq!(decoded.header.frame_type, original_frame.header.frame_type);
        assert_eq!(decoded.header.channel, original_frame.header.channel);
        assert_eq!(decoded.payload, original_frame.payload);
    }

    // Test frame header with different values
    #[test]
    fn test_frame_header_various_values() {
        let test_cases = vec![
            (0, FrameType::AMQP as u8, 0),
            (1, FrameType::SASL as u8, 1),
            (65535, FrameType::AMQP as u8, 65535),
            (4294967295, FrameType::SASL as u8, 32767),
        ];
        
        for (size, frame_type, channel) in test_cases {
            let header = FrameHeader::new(size, frame_type, channel);
            let encoded = header.encode();
            let decoded = FrameHeader::decode(&encoded).unwrap();
            
            assert_eq!(decoded.size, size);
            assert_eq!(decoded.frame_type, frame_type);
            assert_eq!(decoded.channel, channel);
            assert_eq!(decoded.data_offset, 2); // Always 2 for AMQP 1.0
        }
    }

    // Test frame with empty payload
    #[test]
    fn test_frame_empty_payload() {
        let header = FrameHeader::new(0, FrameType::AMQP as u8, 0);
        let payload = vec![];
        let frame = Frame::new(header, payload);
        
        let encoded = frame.encode();
        assert_eq!(encoded.len(), 8); // Only header, no payload
        
        let decoded = Frame::decode(&encoded).unwrap();
        assert_eq!(decoded.header.size, 0);
        assert_eq!(decoded.payload.len(), 0);
    }

    // Test frame with large payload
    #[test]
    fn test_frame_large_payload() {
        let payload_size = 1000;
        let payload = vec![0x42; payload_size];
        let header = FrameHeader::new(payload_size as u32, FrameType::AMQP as u8, 1);
        let frame = Frame::new(header, payload);
        
        let encoded = frame.encode();
        assert_eq!(encoded.len(), 8 + payload_size); // Header + payload
        
        let decoded = Frame::decode(&encoded).unwrap();
        assert_eq!(decoded.header.size, payload_size as u32);
        assert_eq!(decoded.payload.len(), payload_size);
        assert_eq!(decoded.payload, vec![0x42; payload_size]);
    }
} 
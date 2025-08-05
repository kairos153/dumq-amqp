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
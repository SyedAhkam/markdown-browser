#[macro_use]
extern crate protocol_derive;

use std::net::{TcpStream, Shutdown};
use protocol::{
    wire::{
        stream::Connection as WireConnection,
        middleware::pipeline as protocol_pipeline
    },
    Parcel,
    Settings as ProtocolSettings
};

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Handshake;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Hello;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Goodbye {
    pub reason: String,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Url(String);

impl Url {
    pub fn new(address: &str, filename: &str) -> Self {
        Self(format!("{}/{}", address, filename))
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Request {
    pub url: Url
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Markdown(String);

impl Markdown {
    pub fn from(string: String) -> Self {
        Self(string)
    }
}

impl std::fmt::Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Response {
    pub md: Markdown
}

#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum Packet {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
    
    #[protocol(discriminator(0x01))]
    Hello(Hello),
    
    #[protocol(discriminator(0x02))]
    Goodbye(Goodbye),

    #[protocol(discriminator(0x03))]
    Request(Request),

    #[protocol(discriminator(0x04))]
    Response(Response)
}

#[derive(Debug)]
pub struct TcpConnection(TcpStream);

impl TcpConnection {
    pub fn new(address: &str) -> Option<Self> {
        match TcpStream::connect(address) {
            Ok(stream) => Some(Self(stream)),
            Err(e) => {
                eprintln!("failed to connect with tcp server: {:?}", e);
                None
            }
        }
    }

    pub fn from_stream(stream: TcpStream) -> Self { Self(stream) }

    pub fn into_inner(self) -> TcpStream { self.0 } 
}

#[derive(Debug)]
pub struct ProtocolConnection<P: Parcel, S: std::io::Read + std::io::Write>(WireConnection<P, S>);

impl <P, S>ProtocolConnection<P, S>
    where P: Parcel, S: std::io::Read + std::io::Write
{
    pub fn new(stream: S) -> Self {
        let wire_connection = WireConnection::new(
            stream,
            protocol_pipeline::default(),
            ProtocolSettings::default(),
        );

        return Self(wire_connection)
    }

    pub fn send_packet(&mut self, packet: &P) {
        self.0.send_packet(packet).expect("failed to send packet");
    }

    pub fn receive_packet(&mut self) -> Option<P> {
        self.0.receive_packet().expect("failed to receive packet")
    }

    pub fn into_inner(self) -> WireConnection<P, S> { self.0 }
}

use async_std::{net::{UdpSocket, TcpStream}, io::WriteExt};

use crate::minet;
use crate::minet::packets::*;
use std::{fmt, fmt::Debug, any::{Any, TypeId}, io::{Write, Read, self}};

pub trait DataClone: Any + DataCloneBox {
    fn serialize(&self, buf: &mut minet::Writer);   
}
pub trait DataCloneBox {
    fn clone_box(&self) -> Box<dyn DataClone>;
    fn debug_fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}

impl<T> DataClone for T where T: Any + Data {
    fn serialize(&self, buf: &mut minet::Writer) {
        <Self as Data>::serialize(self, buf);
    }
}
impl<T> DataCloneBox for T where T: Any + Data {
    fn clone_box(&self) -> Box<dyn DataClone> {
        Box::new(self.clone())
    }

    fn debug_fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        <Self as Debug>::fmt(self, f)
    }
}
impl Clone for Box<dyn DataClone> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
impl Debug for Box<dyn DataClone> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug_fmt(f)
    }
}

/// Represents any kind of data
/// that can be marshalled and unmarshalled
/// into byte slices.
pub trait Data : Clone + Debug {
    fn serialize(&self, buf: &mut minet::Writer);
    fn deserialize(data: &[u8]) -> Option<Self>;
}

/// Represents data that can be
/// directly sent as a UDP message
/// or can be wrapped with a length
/// to be sent as a TCP message.
pub trait Message : Data {}

#[derive(Debug, Clone)]
pub struct PacketMessage {
    packet: Box<dyn DataClone>,
    pub compressed: bool
}

impl PacketMessage {
    pub fn new<T: Packet + 'static>(packet: T, compressed: bool) -> Self {
        Self {
            packet: Box::new(packet),
            compressed
        }
    }

    pub fn id(&self) -> u8 {
        if self.packet.type_id() == TypeId::of::<ConnectPacket>() {
            3
        } else {
            panic!("invalid packet of type {:?}", self.packet.type_id())
        }
    }
}

impl Data for PacketMessage {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u8(self.id());
        
        let mut packet_data = minet::Writer::new();
        self.packet.serialize(&mut packet_data);
        let len = packet_data.0.len();
        buf.u16(len as u16);

        buf.bool(self.compressed);
        if self.compressed {
            let compressed_data = lz4_flex::block::compress(&packet_data.0);
            buf.bytes(&compressed_data);
        } else {
            buf.bytes(&packet_data.0);
        }
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        let id = reader.u8()?;
        let len = reader.u16()? as usize;
        let compressed = reader.bool()?;

        let original_data = reader.bytes_remaining();
        let mut decompressed_data_buf = Vec::new();
        let decompressed_data = if compressed {
            decompressed_data_buf = match lz4_flex::block::decompress(original_data, len) {
                Err(e) =>  { eprintln!("error decompressing: {:?}", e); return None },
                Ok(v) => v
            };
            &decompressed_data_buf
        } else {
            original_data
        };

        match id {
            3 => {
                ConnectPacket::deserialize(decompressed_data).map(|packet| 
                    Self {
                        packet: Box::new(packet),
                        compressed 
                    }   
                )
            },
            _ => {
                None
            }
        }
    }
}

impl Message for PacketMessage {}

#[derive(Debug, Clone)]
pub struct FrameworkMessage {
    inner: Box<dyn DataClone>,
}

impl FrameworkMessage {
    pub fn new<T: Framework + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner)
        }
    }
}

impl Data for FrameworkMessage {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u8(0xfe);
        self.inner.serialize(buf);
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        if reader.u8() != Some(0xfe) {
            return None;
        }
        match reader.peek_u8() {
            Some(1) => {
                Some(Self {inner:Box::new(DiscoverHost)})
            },
            Some(3) => {
                Some(Self { inner: Box::new(RegisterUDP::deserialize(reader.bytes_remaining())?) })
            },
            Some(4) => {
                Some(Self { inner: Box::new(RegisterTCP::deserialize(reader.bytes_remaining())?) })
            }
            None| Some(_) => None,
        }
    }
}

impl Message for FrameworkMessage {}

pub async fn send_udp<T: Message>(socket: UdpSocket, data: T) -> io::Result<usize> {
    let mut buf = minet::Writer::new();
    data.serialize(&mut buf);
    socket.send(&buf.0).await
}

pub async fn send_tcp<T: Message>(mut socket: TcpStream, data: T) -> io::Result<usize> {
    let mut buf = minet::Writer::new();
    data.serialize(&mut buf);

    let mut buf_tcp = minet::Writer::new();
    buf_tcp.u16(buf.0.len() as u16);
    buf_tcp.bytes(&buf.0);

    socket.write(&buf_tcp.0).await
}

pub fn parse_udp(data: &[u8]) -> Option<Box<dyn DataClone>> {
    if data.len() == 0 {
        return None
    } else if data[0] == 0xfe {
        return FrameworkMessage::deserialize(data).map(|b| {
            let b: Box<dyn DataClone> = Box::new(b);
            b
        });
    } else {
        return PacketMessage::deserialize(data).map(|b| {
            let b: Box<dyn DataClone> = Box::new(b);
            b
        });
    }
}

#[test]
fn test_framework_message() {
    assert!(FrameworkMessage::deserialize(&[0xfe, 0x1]).is_some());
    assert!(FrameworkMessage::deserialize(&[0xfe, 0x3, 0, 0, 0, 5]).is_some());
    assert!(FrameworkMessage::deserialize(&[0xfe, 0x4, 0, 0, 0, 5]).is_some());
}

#[test]
fn test_packet_message() {
    let data = &[0x03,0x00,0x44,0x01,0xf0,0x35,0x00,0x00,0x00,0x87,0x01,0x00,0x08,0x6f,0x66,0x66,0x69,0x63,0x69,0x61,0x6c,0x01,0x00,0x05,0x61,0x6c,0x6c,0x65,0x6e,0x01,0x00,0x05,0x65,0x6e,0x5f,0x55,0x53,0x01,0x00,0x0c,0x79,0x33,0x2f,0x70,0x33,0x58,0x37,0x77,0x45,0x74,0x6b,0x3d,0x4a,0xef,0x2f,0x79,0x87,0x17,0x4f,0x99,0x00,0x00,0x00,0x00,0xbd,0x7a,0xa1,0xb2,0x00,0xff,0x76,0xa6,0xff,0x00];
    assert!(PacketMessage::deserialize(data).is_some());
}

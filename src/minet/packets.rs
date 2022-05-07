use crate::minet::data::*;
use crate::minet;

/// Represents data that can be
/// embedded inside of a `PacketMessage`
/// to constitute a `Message`.
pub trait Packet : Data {}

#[derive(Clone, Debug)]
pub struct ConnectPacket {
    pub version_build: u32,
    pub version_type: String,
    pub player_name: String,
    pub locale: String,
    pub usid: String,
    pub uuid: uuid::Uuid,
    pub mobile: bool,
    pub color: u32,
}

impl Data for ConnectPacket {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u32(self.version_build);
        buf.str(&self.version_type);
        buf.str(&self.player_name);
        buf.str(&self.locale);
        buf.str(&self.usid);

        let uuid_bytes = self.uuid.as_bytes();
        buf.bytes(uuid_bytes);
        let cipher = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
        buf.u64(cipher.checksum(uuid_bytes) as u64);

        buf.bool(self.mobile);
        buf.u32(self.color);
        buf.u8(0); // no mods
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        let version_build = reader.u32()?;
        let version_type = reader.str()?.to_string();
        let player_name = reader.str()?.to_string();
        let locale = reader.str()?.to_string();
        let usid = reader.str()?.to_string();
        let uuid_bytes = reader.bytes(16)?;
        let uuid_bytes = <&[u8] as TryInto<[u8; 16]>>::try_into(uuid_bytes).ok()?;
        let uuid_checksum = reader.u64()?;
        let mobile = reader.bool()?;
        let color = reader.u32()?;
        // ignore mods

        Some(Self {
            version_build, version_type: version_type.to_string(),
            player_name: player_name.to_string(), locale: locale.to_string(),
            usid: usid.to_string(),
            uuid: uuid::Uuid::from_bytes(uuid_bytes),
            mobile, color
        })
    }
}
impl Packet for ConnectPacket {}

/// Represents data that can be
/// embedded inside of a `FrameworkMessage`
/// to constitute a `Message`.
pub trait Framework : Data {}

#[derive(Clone, Debug)]
pub struct RegisterUDP {
    pub id: u32
}

impl Data for RegisterUDP {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u8(3);
        buf.u32(self.id);
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        if reader.u8() != Some(3) {
            return None
        }

        let id = reader.u32()?;

        Some(Self { id })
    }
}
impl Framework for RegisterUDP {}

#[derive(Clone, Debug)]
pub struct RegisterTCP {
    pub id: u32
}

impl Data for RegisterTCP {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u8(4);
        buf.u32(self.id);
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        if reader.u8() != Some(4) {
            return None
        }

        let id = reader.u32()?;

        Some(Self { id })
    }
}
impl Framework for RegisterTCP {}

#[derive(Clone, Debug)]
pub struct DiscoverHost;

impl Data for DiscoverHost {
    fn serialize(&self, buf: &mut minet::Writer) {
        buf.u8(1);
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        let mut reader = minet::Reader::new(data);
        if reader.u8() != Some(1) {
            return None
        }

        Some(Self)
    }
}

#[test]
fn test_connect_packet() {
    let data = &[0x03,0x00,0x44,0x01,0xf0,0x35,0x00,0x00,0x00,0x87,0x01,0x00,0x08,0x6f,0x66,0x66,0x69,0x63,0x69,0x61,0x6c,0x01,0x00,0x05,0x61,0x6c,0x6c,0x65,0x6e,0x01,0x00,0x05,0x65,0x6e,0x5f,0x55,0x53,0x01,0x00,0x0c,0x79,0x33,0x2f,0x70,0x33,0x58,0x37,0x77,0x45,0x74,0x6b,0x3d,0x4a,0xef,0x2f,0x79,0x87,0x17,0x4f,0x99,0x00,0x00,0x00,0x00,0xbd,0x7a,0xa1,0xb2,0x00,0xff,0x76,0xa6,0xff,0x00];
    assert!(ConnectPacket::deserialize(data).is_some());
}
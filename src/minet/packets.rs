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
    let data = &[0, 0, 0, 135, 1, 0, 8, 111, 102, 102, 105, 99, 105, 97, 108, 1, 0, 5, 97, 108, 108, 101, 110, 1, 0, 5, 101, 110, 95, 85, 83, 1, 0, 12, 121, 51, 47, 112, 51, 88, 55, 119, 69, 116, 107, 61, 74, 239, 47, 121, 135, 23, 79, 153, 0, 0, 0, 0, 189, 122, 161, 178, 0, 255, 118, 166, 255, 0];
    assert!(ConnectPacket::deserialize(data).is_some());

    let mut encoded = minet::Writer::new();
    Data::serialize(&ConnectPacket { 
        version_build: 135,
        version_type: "rustbot".to_owned(),
        player_name: "allen".to_owned(),
        locale: "en-US".to_owned(),
        usid: "AAAAAAAA".to_owned(),
        uuid: uuid::Uuid::new_v4(),
        mobile: false,
        color: 0x00ff00ff
    }, &mut encoded);
    assert!(ConnectPacket::deserialize(&encoded.0).is_some());
}
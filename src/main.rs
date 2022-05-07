mod minet;
use std::io::{self, Write, Read};
use std::net::{UdpSocket, TcpStream};
use rand::Rng;
use uuid::Uuid;

fn main() -> io::Result<()> {
    let mut udp_socket = UdpSocket::bind("0.0.0.0:5001")?;
	udp_socket.connect("0.0.0.0:6567")?;
    let mut tcp_socket = TcpStream::connect("0.0.0.0:6567")?;

	let mut rng = rand::thread_rng();

	// UDP ping
	{
		let mut msg = minet::Writer::new();
		msg.bytes(&[(-2i8) as u8, 1]);
		udp_socket.send(&msg.0)?;
	}

	// TCP register 
	let id = {
		let mut buf = [0u8; 16];
		tcp_socket.read(&mut buf)?;
		((buf[4] as u32) << 24) + ((buf[5] as u32) << 16)+ ((buf[6] as u32) << 8) + (buf[7]) as u32
	} as i32;

	// register UDP
	{
		let mut msg = minet::Writer::new();
		msg.bytes(&[(-2i8) as u8, 3]);
		msg.i32(id);
		udp_socket.send(&msg.0)?;
	}
		
	let mut buf = [0u8; 16];
	println!("waiting for tcp reply...");
	tcp_socket.read(&mut buf)?; // wait for registration reply...
	println!("tcp reply: {:?}", buf);

	println!("waiting for udp reply...");
	udp_socket.recv(&mut buf)?;

	// send ConnectPacket
	{
		let mut msg = minet::Writer::new();
		msg.u8(0x00);
		msg.u8(0x4a);
		msg.i32(-1);
		msg.str("null");
		msg.str("robot");
		msg.str("en-US");
		msg.str("AAAAAAAA"); // USID
		
		let uuid = Uuid::new_v4();
		let bytes = uuid.as_bytes();
		msg.bytes(bytes);
		let cipher = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
		let checksum = cipher.checksum(bytes);
		msg.i32(checksum as i32);

		msg.bool(false);
		msg.i32(0);
		msg.u8(0);

		tcp_socket.write(&msg.0)?;
	}

	loop {
		tcp_socket.read(&mut buf)?;
	}
		
    Ok(())
}

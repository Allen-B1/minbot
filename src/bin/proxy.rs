use std::{io::{self}, net::{SocketAddr, SocketAddrV4}, error::Error, fmt::Display};
use async_std::{net::{TcpStream, TcpListener, UdpSocket}, io::{ReadExt, WriteExt}};
#[path = "../minet/mod.rs"]
mod minet;

#[derive(Debug)]
struct Err<E: Error>(String, E);

impl<E: Error> Display for Err<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<E: Error> Error for Err<E> {}

fn print_data(protocol: &'static str, direction: &'static str, raw_data: &[u8]) {
    let data = minet::parse_udp(raw_data);
    println!("{} {}", protocol, direction);
    println!("\traw: {}", raw_data.iter().map(|s| format!("{:02x} ", s)).collect::<String>());
    if let Some(data) = data {
        println!("\tparsed: {}", format!("{:#?}", data).replace("\n", "\n\t"));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr: SocketAddr = "127.0.0.1:6567".parse().unwrap();
    let udp_socket = UdpSocket::bind("0.0.0.0:5001").await?;
    let tcp_socket = TcpListener::bind("0.0.0.0:5001").await?;
    let (mut tcp_client_stream, client_addr) = tcp_socket.accept().await?;
    eprintln!("client connected from {}", client_addr);
    let mut tcp_server_stream = TcpStream::connect(server_addr).await?;


    let mut buf = [0u8; 16384];
    let mut buf_tcp = [0u8; 16384];
    let mut buf_tcp2 = [0u8; 16384];
    loop {
        tokio::select! {
            tcp_res = tcp_client_stream.read(&mut buf_tcp) => {
                let len = tcp_res?;
                if len == 0 {
                    break
                }
                tcp_server_stream.write(&buf_tcp[..len]).await.map_err(|e| Err("tcp server write".to_string(), e))?;

                let message_data = &buf_tcp[2..len];

                eprintln!("tcp message from client");
                print_data("TCP", "client -> server", message_data);
            },
            tcp_res2 = tcp_server_stream.read(&mut buf_tcp2) => {
                let len = tcp_res2?;
                tcp_client_stream.write(&buf_tcp2[..len]).await.map_err(|e| Err("tcp client write".to_string(), e))?;

                let message_data = &buf_tcp2[2..len];

                eprintln!("tcp message from server");
                print_data("TCP", "server -> client", message_data);
            },
            udp_res = udp_socket.recv_from(&mut buf) => {
                let (len, udp_addr) = udp_res?;
                if udp_addr == server_addr {
//                    eprintln!("udp message from {}, assuming to be server", udp_addr);
                    udp_socket.send_to(&buf[..len], client_addr).await.map_err(|e| Err("udp client write".to_string(), e))?;
                    print_data("UDP", "server -> client", &buf[..len]);
                } else { // message from client
//                    eprintln!("udp message from {}, assuming to be client", udp_addr);
                    udp_socket.send_to(&buf[..len], server_addr).await.map_err(|e| Err("udp server write".to_string(), e))?;
                    print_data("UDP", "client -> server", &buf[..len]);
                }
            }
        }
    }

    eprintln!("client disconnected");
    Ok(())
}
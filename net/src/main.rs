//! Filename: main.rs
//! Description: 监听未使用的 TCP/IP 端口
//! Date: 2024/06/20 08:28:25

use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Error> {
    println!("Hello, net!");
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; // 阻塞，直到被请求
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}

extern crate rustc_serialize;
extern crate bincode;

mod message;

use message::{Message};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}; 

fn main() {
    let mut buf: [u8; 1024]  = [0; 1024];

    let local_ip = Ipv4Addr::new(0,0,0,0);
    let port = 12345;
    let local_addr = SocketAddrV4::new(local_ip, port);

    // Create a socket and attempt to bind it to the local address
    let socket: UdpSocket;
    match UdpSocket::bind(local_addr) {
        Ok(s) => {
            socket = s;
            println!("Bind successful to port {}", port);
        },
        Err(e) => { 
            println!("Bind error: {}", e); 
            return;
        }
    }

    loop {
        match socket.recv_from(&mut buf) {
            Ok((num_bytes, _)) => {
                let msg = Message::decode(&buf[0..num_bytes]);
                println!("Got: {:?}", msg);
            },
            Err(e) => {
                println!("Receive error: {}", e);
                drop(socket);
                return;
            }
        }
    }
}

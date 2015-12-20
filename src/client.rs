extern crate rustc_serialize;
extern crate bincode;

mod message;

use message::{Message};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket}; 

fn main() {
    // Define IP addresses and port
    let local_ip = Ipv4Addr::new(0,0,0,0); // any address
    let serv_ip  = Ipv4Addr::new(10, 5, 150, 241); // remote machine IP address
    let port = 12345;
    
    // Combine IPs and port together into single address
    let local_addr = SocketAddrV4::new(local_ip, port);
    let serv_addr  = SocketAddrV4::new(serv_ip, port);

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

    for i in 0..1000000 {
        let msg = Message::new(i, "Hello there!");
        let mut buf = {
            let mut encoded = Message::encode(&msg).unwrap();
            (&mut encoded[..]).to_owned()
        };

        // Attempt to send buffer of data to the remote machine
        match socket.send_to(&mut buf, serv_addr) {
            Ok(s) => {
                println!("Sent {} bytes to {:?}", s, serv_addr);
                println!("{:?}", msg);
            },
            Err(e) => {
                println!("Send error: {}", e);
                drop(socket);
                return;
            }
        }
    }

    // Release the socket from use
    drop(socket);
}

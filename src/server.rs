use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}; 

fn main() {
    let mut buf = [0; 10];

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
        // Wait to receive a message
        let src_addr: SocketAddr;
        let num_bytes;
        match socket.recv_from(&mut buf) {
            Ok((n, src)) => {
                println!("Got {} bytes from {}", n, src);
                println!("Got: {:?}", buf);
                src_addr = src;
                num_bytes = n;
            },
            Err(e) => {
                println!("Receive error: {}", e);
                drop(socket);
                return;
            }
        }

        // Reverse the bytes in the received message
        let mut buf = &mut buf[..num_bytes];
        buf.reverse();

        // Attempt to send the reversed message back to the sender
        match socket.send_to(&mut buf, src_addr) {
            Ok(s) => {
                println!("Sent {} bytes to {:?}", s, src_addr);
                println!("Sent: {:?}", buf);
            },
            Err(e) => {
                println!("Send error: {}", e);
                drop(socket);
                return;
            }
        }
        println!("");
    }
}

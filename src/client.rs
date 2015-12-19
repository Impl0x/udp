use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket}; 

fn main() {
    // Define IP addresses and port
    let local_ip = Ipv4Addr::new(0,0,0,0); // any address
    let serv_ip  = Ipv4Addr::new(10, 5, 150, 241); // remote machine IP address
    let port = 12345;
    
    // The buffer being sent over the network
    let mut buf: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
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

    // Attepmt to send buffer of data to the remote machine
    match socket.send_to(&mut buf, serv_addr) {
        Ok(s) => {
            println!("Sent {} bytes to {:?}", s, serv_addr);
            println!("Sent: {:?}", buf);
        },
        Err(e) => {
            println!("Send error: {}", e);
            drop(socket);
            return;
        }
    }

    // Attempt to get a message back from the remote machine
    match socket.recv_from(&mut buf) {
        Ok((num_bytes, src_addr)) => {
            println!("Got {} bytes from {}", num_bytes, src_addr);
            println!("Got: {:?}", buf);
        },
        Err(e) => {
            println!("Receive error: {}", e);
            drop(socket);
            return;
        }
    }

    // Release the socket from use
    drop(socket);
}

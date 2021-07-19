use std::{net::UdpSocket, str::from_utf8, thread};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("error");
    let mut buffer = [0;1024];
    loop {
        let socket_new = socket.try_clone().expect("Unable");
        match socket_new.recv_from(&mut buffer) {
            Ok((num_bytes, src_addr)) => {
                thread::spawn(move || {
                    let send_buffer = &mut buffer[..num_bytes];
                    println!("Received from client: {}", from_utf8(send_buffer).unwrap());
                    let res_str = format!("Recieved this: {}", String::from_utf8_lossy(send_buffer));
                    socket_new.send_to(res_str.as_bytes(), &src_addr).expect("error");
                });
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

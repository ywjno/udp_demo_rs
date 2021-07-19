use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("error");
    socket.connect("127.0.0.1:3000").expect("error");
    println!("socket {:?}", socket.peer_addr());
    socket.send("Hello [from client]".as_bytes()).expect("error");
    let mut buffer = [0;1024];
    match socket.recv_from(&mut buffer) {
        Ok((num_byte, src_addr)) => {
            thread::spawn(move || {
                let get_buffer = &mut buffer[..num_byte];
                println!("{}", std::str::from_utf8(&get_buffer).unwrap());
            });
        }
        Err(e) => {
            println!("Err: {}", e)
        }
    }
}

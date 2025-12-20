use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8080;

fn main() {
    let address = SocketAddrV4::new(IP, PORT);
    let server = TcpListener::bind(address).unwrap();

    println!("Load balancer rodando em {}:{}", IP, PORT);

    for stream in server.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_client_connection(stream);
        });
    }
}

fn handle_client_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let bytes = stream.read(&mut buffer).unwrap();

        if bytes == 0 {
            break;
        }

        match str::from_utf8(&buffer[..bytes]) {
            Ok(request) => {
                println!("Mensagem recebida: {request}");
                stream.write_all(b"Mensagem recebida").unwrap();
            }
            Err(e) => println!("Error: {e}")
        }
    }
}

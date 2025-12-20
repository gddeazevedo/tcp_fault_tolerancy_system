use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Load balancer rodando em 127.0.0.1:8080");

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

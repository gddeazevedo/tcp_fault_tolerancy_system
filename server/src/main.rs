use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::thread;

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Servidor rodando em 127.0.0.1:3000");

    for stream in server.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
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

        match std::str::from_utf8(&buffer[..bytes]) {
            Ok(request) => {
                println!("Processando requisição: {request}");
            }
            Err(e) => println!("Erro ao processar requisição: {e}")
        }
    }
}

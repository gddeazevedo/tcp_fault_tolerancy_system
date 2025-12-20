use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::env;
use rand::Rng;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::rng();

    let id = &args[1];

    let mut client = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Um buffer de 1024 bytes utilizado para armazenar dados recebidos do servidor temporariamente.
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let message: String = format!("ESCRITA|10|123|{}", id);
        client.write_all(message.as_bytes()).unwrap();
        let bytes = client.read(&mut buffer).unwrap();

        match str::from_utf8(&buffer[..bytes]) {
            Ok(response) => println!("Resposta do servidor: {}", response),
            Err(e) => println!("Erro ao converter resposta: {}", e),
        }

        let time_to_sleep = rng.random_range(1..10);
        thread::sleep(std::time::Duration::from_secs(time_to_sleep));
    }
}

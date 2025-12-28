use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::env;
use std::time::Duration;
use rand::Rng;

const RANDOM_LOWER_BOUND: u32 = 2;
const RANDOM_UPPER_BOUND: u32 = 1_000_000;
const SLEEP_LOWER_BOUND: u64 = 20;
const SLEEP_UPPER_BOUND: u64 = 50;
const PROBABILITY_THRESHOLD: f64 = 0.5;


fn main() {
    let args: Vec<String> = env::args().collect();
    let id = &args[1];

    let mut client = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Um buffer de 1024 bytes utilizado para armazenar dados recebidos do servidor temporariamente.
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let (first_number, second_number) = get_random_numbers();
        let message = generate_message(first_number, second_number, id);

        client.write_all(message.as_bytes()).unwrap();
        let bytes = client.read(&mut buffer).unwrap();

        match str::from_utf8(&buffer[..bytes]) {
            Ok(response) => println!("Resposta do loadbalancer: {}", response),
            Err(e) => println!("Erro ao converter resposta: {}", e),
        }

        sleep_execution();
    }
}


fn get_random_numbers() -> (u32, u32) {
    let mut rng = rand::rng();
    let range = RANDOM_LOWER_BOUND..=RANDOM_UPPER_BOUND;
    let first_number = rng.random_range(range.clone());
    let second_number = rng.random_range(range);
    (first_number, second_number)
}

fn generate_message(first_number: u32, second_number: u32, id: &str) -> String {
    let mut rng = rand::rng();
    let probability: f64 = rng.random();

    return if probability < PROBABILITY_THRESHOLD {
        format!("ESCRITA|{}|{}|id:{}", first_number, second_number, id)
    } else {
        format!("LEITURA|id:{}", id)
    };
}


fn sleep_execution() {
    let mut rng = rand::rng();
    let time_to_sleep = rng.random_range(SLEEP_LOWER_BOUND..=SLEEP_UPPER_BOUND);
    thread::sleep(Duration::from_secs(time_to_sleep));
}

mod network;
use network::pop3client::POP3Client;

fn main() {
    println!("Hello, world!");
    let _ = POP3Client::new();
}

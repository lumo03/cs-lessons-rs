use std::io::{self, BufRead, Write};
use std::net::TcpStream;

const SERVER: &str = "10.2.126.105";
const PORT: u16 = 10110;
const USERNAME: &str = "anna";
const PASSWORD: &str = "geheim";

pub struct POP3Client {
    socket: TcpStream,
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl POP3Client {
    pub fn new() -> Self {
        let socket = TcpStream::connect((SERVER, PORT)).unwrap();
        let reader = io::BufReader::new(socket.try_clone().unwrap());
        let writer = io::BufWriter::new(socket.try_clone().unwrap());

        let mut client = POP3Client {
            socket,
            reader,
            writer,
        };

        client.connect();
        client.login();
        client.liste_mails();
        client.close_connection();
        client
    }

    fn connect(&mut self) {
        println!("Attempting connection... ");
        let response = self.receive();
        println!("RECEIVED: {response}");
    }

    fn login(&mut self) {
        println!("Sending login credentials...");
        self.send(format!("USER {USERNAME}\r\n"));
        let response = self.receive();
        println!("RECEIVED: {response}");

        self.send(format!("PASS {PASSWORD}\r\n"));
        let response = self.receive();
        println!("RECEIVED: {response}");

        if !response.starts_with("+OK") {
            println!("Login error");
            std::process::exit(1);
        }
    }

    fn liste_mails(&mut self) {
        println!("Retrieving email list...");
        self.send("LIST\r\n".to_string());
        let response = self.receive();
        if response.starts_with("-ERR") {
            println!("Error retrieving email list");
            std::process::exit(1);
        }
        println!("{response}");
    }

    fn get_mail(&mut self, id: i32) {
        println!("Retrieving email with id {id}...");
    }

    fn close_connection(&mut self) {
        println!("Ending connection");
        self.socket.shutdown(std::net::Shutdown::Both).unwrap();
        println!("Goodbye!");
    }
    /*
    fn receive(&mut self) -> String {
        let mut response = String::new();
        self.reader.read_line(&mut response).unwrap();
        response
    }*/

    fn receive(&mut self) -> String {
        let mut response = String::new();
        // let bytes_read = self.reader.read_line(&mut response).unwrap();
        let _ = self.reader.read_line(&mut response).unwrap();
        response = response.trim_start_matches('\r').to_string();
        // println!("Bytes read: {}", bytes_read);
        response
    }

    fn send(&mut self, message: String) {
        let _ = self.writer.write(message.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }
}

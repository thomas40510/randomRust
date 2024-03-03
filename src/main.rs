use std::io::{Read, Write};
use std::net::TcpStream;

mod vigenere;
mod caesar;
mod rsa;

fn main() {
    // select "receive" or "send" mode, then open / receive a connection
    println!("Would you like to send or receive a message? (send/receive)");
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim();
    if mode == "send" {
        // send mode
        // ask user for the IP address and port number
        println!("Please select the port to use (e.g. 8080):");
        let mut port = String::new();
        std::io::stdin().read_line(&mut port).unwrap();
        let ip = "localhost";
        let port: u16 = port.trim().parse().unwrap();
        // ask user for the message indefinitely
        loop {
            println!("Please enter the message you would like to send:");
            let mut message = String::new();
            std::io::stdin().read_line(&mut message).unwrap();
            let message = message.trim();
            // encryption is rsa
            let rsa = rsa::RSA::new(3233, 17, 2753);
            let encrypted = rsa.encrypt_string(message);
            // send the message
            let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
            for c in encrypted {
                stream.write(&c.to_be_bytes()).unwrap();
            }
        }
    } else if mode == "receive" {
        // receive mode
        // ask user for the port number
        println!("Please enter the port number you would like to listen on:");
        let mut port = String::new();
        std::io::stdin().read_line(&mut port).unwrap();
        let port: u16 = port.trim().parse().unwrap();
        
        // encryption is rsa
        let rsa = rsa::RSA::new(3233, 17, 2753);
        loop {// receive the message
            let listener = std::net::TcpListener::bind(format!("localhost:{}", port)).unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 8];
                let mut message = Vec::new();
                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        break;
                    }
                    message.push(u64::from_be_bytes(buffer));
                }
                // decryption
                let decrypted = rsa.decrypt_string(&message);

                println!("Received message: {}", decrypted);
            }
        }
    }
}
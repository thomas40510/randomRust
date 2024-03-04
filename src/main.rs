use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod vigenere;
mod caesar;
mod rsa;

// global variable to store the messages
static mut LASTMSG: Vec<Vec<u64>> = Vec::new();

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
    } else if mode == "middle" {
        // ask for the port
        println!("Please enter the port number you would like to listen on:");
        let mut port = String::new();
        std::io::stdin().read_line(&mut port).unwrap();
        let port: u16 = port.trim().parse().unwrap();
        
        // man-in-the-middle: receive the message and forward it
        // don't decipher it, and don't impact the time it takes to send the message
        
        loop {
            let listener = std::net::TcpListener::bind(format!("localhost:{}", port)).unwrap();
            let mut message = Vec::new();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 8];
                message.clear();
                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        break;
                    }
                    message.push(u64::from_be_bytes(buffer));
                }
                // forward the message
                let mut stream = TcpStream::connect("localhost:8080").unwrap();
                for c in &message {
                    stream.write(&c.to_be_bytes()).unwrap();
                }
            }
        }
    } else if mode=="mid" {
        // open connection to port 5050
        std::thread::spawn(|| {
            loop {
                listen_and_forward("localhost", 8080);
            }
        }).join().unwrap();
    }
}

fn listen_and_forward(ip: &str, port: u16) -> (){
    let listener = std::net::TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    let message = receive_message(listener);
    println!("Received message: {}", message.iter().map(|c| *c as u8 as char).collect::<String>());
    // forward the message
    fwd_message(message, ip, 8080);
    //display_on_thread(&message);
}

fn receive_message(listener: TcpListener) -> Vec<u64> {
    let mut message = Vec::new();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 8];
        while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            message.push(u64::from_be_bytes(buffer));
        }
        break;
    }
    message
}

fn fwd_message(message: Vec<u64>, ip: &str, port: u16) -> (){
    // forward the message
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
    for c in &message {
        stream.write(&c.to_be_bytes()).unwrap();
    }
}

fn display_on_thread(message: &Vec<u64>) -> (){
    // execute a println! on a separate thread
    let message = message.clone();
    std::thread::spawn(move || {
        println!("Received message: {}", message.iter().map(|c| *c as u8 as char).collect::<String>());
    });
}
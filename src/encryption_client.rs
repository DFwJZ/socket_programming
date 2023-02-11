use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

use openssl::ssl::{SslConnector, SslMethod};

fn main() {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut ssl_stream = connector.connect("localhost", stream).unwrap();

    let mut credentials = String::new();

    // Read user input of username and password
    println!("Enter your username: ");
    io::stdin().read_line(&mut credentials).unwrap();
    let username = credentials.trim().to_string();

    credentials = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut credentials).unwrap();
    let password = credentials.trim().to_string();

    // Combine username and password with a space separator
    let credentials = format!("{} {}", username, password);

    ssl_stream.write(credentials.as_bytes()).unwrap();
    // flush any buffered data(buf: message in this case)
    // no data should be left in the message after the flush()
    ssl_stream.flush().unwrap_or_else(|error| {
        println!("Error sending responses: {:?}", error);
    }); 

    if ssl_stream.flush().is_ok() {
        println!("username and password are sent to server");
    }

    let mut buffer = [0; 512];
    let bytes_read = ssl_stream.read(&mut buffer).unwrap();
        
    // split the buffer into two messages based on newline char
    let message_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    // spliting the message by the newline char will remove the newline char from original sent message from server
    let message = message_string.split("\r\n").collect::<Vec<_>>();

    let response = message[0];
    let client_addr = message[1];

    println!("respnse: {}\nclient_address: {}", response, client_addr);

    if response == "Authentication successful!" {
        loop {
            // read message from the user
            let mut message = String::new();
            println!("Enter message to send to server: ");
            io::stdin().read_line(&mut message).unwrap();
            message = message.trim().to_string();

            // sned message to server
            ssl_stream.write(message.as_bytes()).unwrap();
            ssl_stream.flush().unwrap_or_else(|error| {
                println!("Error sending message: {:?}", error);
            });

            if ssl_stream.flush().is_ok() {
            println!("Message sent to server");
            }

                    // read response from the server
        let mut buffer = [0; 512];
        let bytes_read = ssl_stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        println!("Response from server: {}", response);
        }
    } else {
        println!("Authentication failed");
    }
}

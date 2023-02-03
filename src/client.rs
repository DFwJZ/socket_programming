use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut message = String::new();

    // Read user input from standard input (stdin)
    println!("Enter your message: ");
    io::stdin().read_line(&mut message).unwrap();

    stream.write(message.as_bytes()).unwrap();
    // flush any buffered data(buf: message in this case)
    // no data should be left in the message after the flush()
    stream.flush().unwrap_or_else(|error|{
        println!("Error sending respons: {:?}", error);
    });

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("Response: {}", response);
}
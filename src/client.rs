use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
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

    stream.write(credentials.as_bytes()).unwrap();
    // flush any buffered data(buf: message in this case)
    // no data should be left in the message after the flush()
    stream.flush().unwrap_or_else(|error| {
        println!("Error sending responses: {:?}", error);
    }); 

    if stream.flush().is_ok() {
        println!("username and password are sent to server");
    }

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
        
    // split the buffer into two messages based on newline char
    let message_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    // spliting the message by the newline char will remove the newline char from original sent message from server
    let message = message_string.split("\r\n").collect::<Vec<_>>();

    let response = message[0];
    let client_addr = message[1];

    println!("respnse: {}\nclient_address: {}", response, client_addr);

    if response == "Authentication successful!" {

        // read message from the user
        let mut message = String::new();
        println!("Enter message to send to server: ");
        io::stdin().read_line(&mut message).unwrap();
        message = message.trim().to_string();

        // sned message to server
        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap_or_else(|error| {
            println!("Error sending message: {:?}", error);
        });

        if stream.flush().is_ok() {
            println!("Okay!")
        }

        // read confirmation from server
        let mut response1 = String::new();
        stream.read_to_string(&mut response1).unwrap();

        println!("Client: Response from server: {}", response1);
    } else {
        println!("I am getting {}", response);
        println!("Authentication failed.");
    }
}
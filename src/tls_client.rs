use std::io::prelude::*;
use std::net::TcpStream;
use std::io;
use native_tls::TlsConnector;

fn main() {
    // Create a new SSL connector
    let connector = TlsConnector::new().unwrap();

     // control transfer
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap(); 

    let mut stream = match connector.connect("127.0.0.1", stream) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error while establishing SSL connection: {:?}", e);
            return;
        }
    };

    // Read user input of username and password
    let mut credentials = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut credentials).unwrap();
    let username = credentials.trim().to_string();

    let password = rpassword::prompt_password("Enter your password: ").unwrap();
    let password = password.trim().to_string();

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

    if message_string.len() == 0 {
        println!("Something wrong with the server....");
        return;
    }
    // spliting the message by the newline char will remove the newline char from original sent message from server
    let message = message_string.split("\r\n").collect::<Vec<_>>();

    let response = message[0];
    // let client_addr = message[1];    
    // println!("respnse: {}\nclient_address: {}", response, client_addr);

    if response == "Authentication successful!" {
       
        loop {
            println!("Enter message to send to server: ");
            // read message from the user
            let mut message = String::new();

            io::stdin().read_line(&mut message).unwrap();
            message = message.trim().to_string();
            
            if message.len() == 0 {
                continue;
            } 
            // sned message to server
            stream.write(message.as_bytes()).unwrap();

            stream.flush().unwrap_or_else(|error| {
                println!("Error sending message: {:?}", error);
            });

            // read confirmation from server
            let mut response_message = [0; 512];
            let bytes_read_message = stream.read(&mut response_message).unwrap();
            let response_message_string = String::from_utf8_lossy(&response_message[..bytes_read_message]).to_string();
            if response_message_string == "disconnect" {
                println!("Disconnected from the server...");
                break;
            }
            println!("Client: Response from server: {}", response_message_string);
        }
    } else {
        println!("I am getting {}", response);
        println!("Authentication failed.");
    }
}
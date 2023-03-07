use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::thread;

fn handle_server(mut conn: TcpStream) {
    loop {
        let mut buf = [0; 512];
        match conn.read(&mut buf) {
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buf[..bytes_read]).trim().to_string();
                if message == "disconnect" {
                    println!("Disconnected from the server...");
                    break;
                }
                println!("Client: Response from server: {}", message);
            }
            Err(error) => {
                eprintln!("Error reading from server: {:?}", error);
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut credentials = String::new();

    // Read user input of username and password
    println!("Enter your username: ");
    io::stdin().read_line(&mut credentials)?;
    let username = credentials.trim().to_string();

    credentials = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut credentials)?;
    let password = credentials.trim().to_string();

    // Combine username and password with a space separator
    let credentials = format!("{} {}", username, password);

    stream.write_all(credentials.as_bytes()).unwrap();
    // flush any buffered data (credentials in this case)
    stream.flush().unwrap();

    if stream.flush().is_ok() {
        println!("Username and password sent to server");
    }

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let message_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    let message = message_string.split("\r\n").collect::<Vec<_>>();
    let response = message[0];
    let client_addr = message[1];

    println!("Response: {}\nClient address: {}", response, client_addr);

    if response == "Authentication successful!" {
        let server_thread = thread::spawn({
            let stream_copy = stream.try_clone()?;
            move || {
                handle_server(stream_copy);
            }
        });
        
        loop {
            // Read message from the user
            let mut message = String::new();
            println!("Enter message to send to server: ");
            io::stdin().read_line(&mut message)?;
            message = message.trim().to_string();

            // Send message to server
            stream.write_all(message.as_bytes())?;
            stream.flush()?;

            if stream.flush().is_ok() {
                println!("Okay!")
            }
            if message == "disconnect" {
                break;
            }
        }
        server_thread.join().unwrap();
    } else {
        println!("I am getting {}", response);
        println!("Authentication failed.");
    }

    Ok(())
}

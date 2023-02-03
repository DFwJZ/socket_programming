use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

/*
used to handle incoming connections from clients.
It takes a TcpStream as its argument, hich represents a connection to a client.
*/
fn handle_client(mut stream: TcpStream) {
    // creates a mutable buffer with a size of 512 bytes, initialized with zeros.
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // The .. syntax is the range syntax, which represents an inclusive range of all elements in the array.
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

    let message = String::from_utf8_lossy(&buffer[..]);
    let message = message.trim_end_matches("\r\n\r\n");
    let uppercase_message = message.to_ascii_uppercase();


    println!("Replied with: {}", uppercase_message);
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uppercase_message);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        // 'match' similar to switch in Java/Python
        match stream {
            Ok(stream) => {
                println!("Accepted connection from {:?}", stream.peer_addr().unwrap());
                let handle_client_thread = thread::spawn(move || {
                    handle_client(stream)
                });
                handle_client_thread.join().unwrap()
            }
            Err(e) => {
                 println!("Error: {}", e);
            }
        }
    }
}

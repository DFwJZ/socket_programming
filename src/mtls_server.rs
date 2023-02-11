// extern crate openssl;
// #[macro_use]
// extern crate lazy_static;
// use lazy_static::lazy_static;
// use std::sync::Arc;
// use std::collections::HashMap;
// use std::sync::Mutex;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// use std::thread;
// use openssl::ssl::{SslAcceptor, SslStream, SslMethod, SslFiletype};

// lazy_static! {
//     static ref USERS: Mutex<HashMap<String, String>> = Mutex::new([("user1".to_string(), "password1".to_string()),
//                                             ("user2".to_string(), "password2".to_string())].iter().cloned().collect());

//     static ref TOKENS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
// }

// fn handle_client(stream: SslStream<TcpStream>) {
//     let users = match USERS.lock() {
//         Ok(guard) => guard,
//         Err(e) => {
//             println!("Error locking users: {}", e);
//             return;
//         }
//     };

//     let mut tokens = match TOKENS.lock() {
//             Ok(guard) => guard,
//             Err(e) => {
//                 println!("Error locking tokens: {}", e);
//                 return;
//             }
//         };


//     let mut buffer = [0; 512];
//     // read the client's message
//     let bytes_read = stream.read(&mut buffer).unwrap();
//     let incoming_message = String::from_utf8_lossy(&buffer[..bytes_read]);
//     let credentials: Vec<&str> = incoming_message.trim().split(" ").collect();

//     let username = credentials[0];
//     let password = credentials[1];

//     if let Some(stored_password) = users.get(username) {
//         if stored_password == password {
//             // generate token
//             let token = format!("{}:{}", username, password);

//             // store token and client address
//             tokens.insert(token, stream.get_ref().peer_addr().unwrap().to_string());

//             // send success message to client
//             let success = "Authentication successful!\r\n".to_string();
//             stream.write(success.as_bytes()).unwrap();
//             stream.flush().unwrap();
            
//             if stream.flush().is_ok() {
//                 println!("Server: server successually authenticated!")
//             }

//             let client_addr = stream.get_ref().peer_addr().unwrap().to_string();
//             let success_with_addr = format!("{}Client address: {}", success, client_addr);
//             stream.write(success_with_addr.as_bytes()).unwrap();
//             stream.flush().unwrap();
            
//             loop {
//                 let mut buffer2 =  [0; 512];
//                 let bytes_read2 = stream.read(&mut buffer2).unwrap();
//                 let incoming_message2 = String::from_utf8_lossy(&buffer2[..bytes_read2]);
//                 println!("Server: Received message from client: {}", incoming_message2);
    
//                 let incoming_message2 = incoming_message2.trim_end_matches("\r\n\r\n");
//                 if incoming_message2.to_lowercase() == "stop" {
//                     println!("User initiated the disconnection!!!!!!");
//                     let stop_message = "disconnect";
//                     stream.write(stop_message.as_bytes()).unwrap();
//                     stream.flush().unwrap();
//                     break;
//                 }

//                 let uppercase_message = incoming_message2.to_ascii_uppercase();

//                 println!("Server: Response message sent by server: {}", uppercase_message);
//                 let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uppercase_message);
//                 stream.write(response.as_bytes()).unwrap();
//                 stream.flush().unwrap();
//             }
//         } else {
//             // send failure message to client
//             let failure = "Server: Password authentication failed".to_string();
//             stream.write(failure.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         }
//     } else {
//         // send failure message to client
//         let failure = "Server: Username authentication failed".to_string();
//         stream.write(failure.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

// fn main() {
    
//     let mut ssl_acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     ssl_acceptor.set_private_key_file("server.pem", SslFiletype::PEM).unwrap();
//     ssl_acceptor.set_certificate_chain_file("server.pem").unwrap();
//     ssl_acceptor.check_private_key().unwrap();
//     let ssl_acceptor = Arc::new(ssl_acceptor.build());

//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     println!("Listening on port 7878...");
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 let acceptor = ssl_acceptor.clone();
//                 let ssl_stream = acceptor.accept(stream).unwrap();
//                 println!("Accepted connection from {}", ssl_stream.get_ref().peer_addr().unwrap());
//                 thread::spawn(|| {
//                     handle_client(ssl_stream)
//                 });
//             },
//             Err(e) => println!("Error: {}", e),
//         }
//     }
// }


#[macro_use]
extern crate lazy_static;
use openssl::ssl::{SslMethod, SslAcceptor};
use std::collections::HashMap;
use std::sync::Mutex;
use std::io::prelude::*;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = Mutex::new(
        [("user1".to_string(), "password1".to_string()), 
         ("user2".to_string(), "password2".to_string())].iter().cloned().collect()
    );
}

fn main() {
    let acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    fn handle_client(mut stream: openssl::ssl::SslStream<TcpStream>) {
        let users = match USERS.lock() {
            Ok(guard) => guard,
            Err(e) => {
                println!("Error locking users: {}", e);
                return;
            }
        };

        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let incoming_message = String::from_utf8_lossy(&buffer[..bytes_read]);
        let credentials: Vec<&str> = incoming_message.trim().split(" ").collect();

        let username = credentials[0];
        let password = credentials[1];

        if let Some(stored_password) = users.get(username) {
            if stored_password == password {
                let success = "Authentication successful!\r\n".to_string();
                stream.write_all(success.as_bytes()).unwrap();
                stream.flush().unwrap();
                println!("Server: Successfully authenticated!");
                
                loop {
                    let mut buffer = [0; 512];
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    let incoming_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Server: Received message from client: {}", incoming_message);
                                    let incoming_message = incoming_message.trim_end_matches("\r\n\r\n");
                if incoming_message.to_lowercase() == "stop" {
                    println!("User initiated the disconnection!");
                    let stop_message = "disconnect";
                    stream.write_all(stop_message.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    break;
                }

                let uppercase_message = incoming_message.to_ascii_uppercase();

                println!("Server: Response message sent by server: {}", uppercase_message);
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uppercase_message);
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else {
            let failure = "Password authentication failed".to_string();
            stream.write_all(failure.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    } else {
        let failure = "Username authentication failed".to_string();
        stream.write_all(failure.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(e) => { println!("{}", e) }
        }
    }
}


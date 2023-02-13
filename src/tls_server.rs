#[macro_use] 
extern crate lazy_static;
extern crate openssl;
extern crate simplelog;
use log::Log;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::fs::File;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use log::{LevelFilter};
use simplelog::*;


lazy_static! {
    static ref USERS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new([("user1".to_string(), "password1".to_string()),
                                            ("user2".to_string(), "password2".to_string()),
                                            ("1".to_string(), "2".to_string())].iter().cloned().collect()));

    static ref TOKENS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn handle_control_channel(stream: TlsStream<TcpStream>, users: Arc<Mutex<HashMap<String, String>>>, tokens: Arc<Mutex<HashMap<String, String>>>) {

    let mut control_stream = stream;
    let users = match users.lock() {
        Ok(guard) => guard,
        Err(e) => {
            println!("Error locking users: {}", e);
            return;
        }
    };

    let mut tokens = match tokens.lock() {
            Ok(guard) => guard,
            Err(e) => {
                println!("Error locking tokens: {}", e);
                return;
            }
        };

    let mut buffer = [0; 512];
    // read the client's message
    let bytes_read = control_stream.read(&mut buffer).unwrap();
    let incoming_message = String::from_utf8_lossy(&buffer[..bytes_read]);
    let credentials: Vec<&str> = incoming_message.trim().split(" ").collect();

    // parse username and password from received credentials
    let username = credentials[0];
    let password = credentials[1];
    log::info!("user: {} password: {} \n", username, password);

    // Validating client's credentials
    if let Some(stored_password) = users.get(username) {
        if stored_password == password {
            // generate token
            let token = format!("{}:{}", username, password);

            // store token and client address
            tokens.insert(token.clone(), control_stream.get_ref().peer_addr().unwrap().to_string());

            // send success message to client
            let success = "Authentication successful!\r\n".to_string();
            control_stream.write(success.as_bytes()).unwrap();
            control_stream.flush().unwrap();
            
            if control_stream.flush().is_ok() {
                println!("Server: server successually authenticated!")
            }
            
            // handling cient request
            if !tokens.contains_key(&token) {
                println!("Error: Invalid token");
                log::error!("Invalid token");
                return;
            }
            
            let mut data_stream = control_stream;
            loop {
                let mut buffer = [0; 512];
                let bytes_read = data_stream
                    .read(&mut buffer)
                    .unwrap_or_else(|e| {
                        println!("Error reading from stream: {:?}", e);
                        log::error!("Error reading from stream: {:?}", e);
                        0
                    });
                
                if bytes_read == 0 {
                    println!("Server: Disconnection detected from the client side.\nListening again on port 7878...");
                    log::info!("Disconnection detected from the client side.\nListening again on port 7878...");
                    break;
                }

                let incoming_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                log::info!("Received message from client: {}", incoming_message);
                println!("Server: Received message from client: {}", incoming_message);

                let incoming_message = incoming_message.trim_end_matches("\r\n\r\n");

                if incoming_message.to_lowercase() == "stop" {
                    println!("User initiated the disconnection!!!!!!\nListening again on port 7878...");
                    log::info!("User initiated the disconnection!!!!!!");
                    log::info!("Listening again on port 7878...");
                    let stop_message = "disconnect";
                    data_stream.write(stop_message.as_bytes()).unwrap();
                    data_stream.flush().unwrap();
                    break;
                }

                let uppercase_message = incoming_message.to_ascii_uppercase();

                println!("Server: Response message sent by server: {}", uppercase_message);
                log::info!("Response message sent by server: {}", uppercase_message);
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uppercase_message);
                data_stream.write(response.as_bytes()).unwrap();
                data_stream.flush().unwrap();
         
                
            }
        } else {    
            // send failure message to client
            let failure = "Server: Password authentication failed".to_string();
            log::error!("Password authentication failed for user {}", username);
            control_stream.write(failure.as_bytes()).unwrap();
            control_stream.flush().unwrap();
        } 
    }else {
        // send failure message to client
        let failure = "Server: Username authentication failed".to_string();
        control_stream.write(failure.as_bytes()).unwrap();
        control_stream.flush().unwrap();
        log::error!("Username authentication failed for user {}", username);
    }
}

fn create_logger()-> Box<dyn Log>{
    let log_file = File::create("server.log").unwrap();
    let config = ConfigBuilder::new()
    .set_time_format_custom(format_description!("[year]:[month]:[day]:[hour]:[minute]:[second].[subsecond]"))
    .set_level_padding(LevelPadding::Left)
    .build();

    let logger = WriteLogger::new(LevelFilter::Info, config, log_file);

    Box::new(logger)
}


fn main() {
    let mut file = File::open("./certs/my_socket_prog.test.pkcs12.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let cert_password = rpassword::prompt_password("Enter your password to start your server: ").unwrap();
    let cert_password = cert_password.trim().to_string();

    let identity = Identity::from_pkcs12(&identity, &cert_password).unwrap();

    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut threads = vec![];
    let logging_handle = create_logger();
    let logging_handle: Box<dyn Log> = Box::new(logging_handle);

    log::set_boxed_logger(logging_handle).unwrap();
    log::set_max_level(LevelFilter::Debug);

    log::info!("Listening on port 7878...");
    println!("Listening again on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                log::info!("Accepted connection from {}", stream.peer_addr().unwrap());
                let users = USERS.clone();
                let tokens = TOKENS.clone();
                let _logging_handle = log::logger();

                let handle = thread::spawn(move || {
                    let ssl_stream = acceptor.accept(stream).unwrap();
                    handle_control_channel(ssl_stream, users, tokens);
                });

                threads.push(handle);
            }
            Err(e) => log::error!("Error: {}", e),
        }
    }

    // Wait for all the threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    // Cleanup the logging handle
    log::logger().flush();

}
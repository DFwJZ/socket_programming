use std::sync::Arc;
use std::vec;
use native_tls::Identity;
use tokio_native_tls::{ TlsStream};
use log::LevelFilter;
use log::Log;
use simplelog::*;
use time::macros::format_description;
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::{ TcpListener as TokioTcpListener, TcpStream as TokioTcpStream},
    fs::File,
};


async fn handle_control_channel(mut control_stream: TlsStream<TokioTcpStream>) {


    let mut buf = [0; 1024];
    let bytes_read = control_stream.read(&mut buf).await.unwrap();
    let bulk_message = String::from_utf8_lossy(&buf[..bytes_read]);

    let credentials: Vec<&str> = bulk_message.trim().split(" ").collect();

    // parse username and password from received credentials
    let username = credentials[0];
    let password = credentials[1];
    println!("{} {}", username, password);
    // log::info!("user: {} password: {} \n", username, password);

    // Validating client's credentials
    // if let Some(stored_password) = users.get(username) {
    //     if stored_password == password {
    // if (username == "1" && password == "2") || (username == "3" && password == "4") {
            // generate token
            // let token = format!("{}:{}", username, password);

            // // store token and client address
            // tokens.insert(token.clone(), control_stream.get_ref().peer_addr().unwrap().to_string());

            // // send success message to client
            // let success = "Authentication successful!\r\n".to_string();
            // control_stream.write(success.as_bytes()).unwrap();
            // control_stream.flush().unwrap();
            
            // if control_stream.flush().is_ok() {
            //     println!("Server: server successually authenticated!")
            // }
            
            // // handling cient request
            // if !tokens.contains_key(&token) {
            //     println!("Error: Invalid token");
            //     log::error!("Invalid token");
            //     return;
            // }
            
            let mut data_stream = control_stream;
      

            loop {
                let mut buf = [0; 1024];
                let bytes_read = data_stream.read(&mut buf).await.unwrap();
                
                
                if bytes_read == 0 {
                    println!("Server: Disconnection detected from the client side.\nListening again on port 7878...");
                    log::info!("Disconnection detected from the client side.\nListening again on port 7878...");
                    break;
                }

                let bulk_message = String::from_utf8_lossy(&buf[..bytes_read]);
                println!("Server: Received message from client: {}", bulk_message);

                log::info!("Received message from client: {}", bulk_message);
                println!("Server: Received message from client: {}", bulk_message);

                let incoming_message = bulk_message.trim_end_matches("\r\n\r\n");

                if incoming_message.to_lowercase() == "stop" {
                    println!("User initiated the disconnection!!!!!!\nListening again on port 7878...");
                    log::info!("User initiated the disconnection!!!!!!");
                    log::info!("Listening again on port 7878...");
                    let stop_message = "disconnect";
                    data_stream.write(stop_message.as_bytes()).await.unwrap();
                    data_stream.flush().await.unwrap();
                    break;
                }

                let uppercase_message = incoming_message.to_ascii_uppercase();

                println!("Server: Response message sent by server: {}", uppercase_message);
                log::info!("Response message sent by server: {}", uppercase_message);
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uppercase_message);
                data_stream.write(response.as_bytes()).await.unwrap();
                data_stream.flush().await.unwrap();
         
                
            }
        // } else {    
        //     // send failure message to client
        //     let failure = "Server: Password authentication failed".to_string();
        //     log::error!("Password authentication failed for user {}", username);
        //     control_stream.write(failure.as_bytes()).unwrap();
        //     control_stream.flush().unwrap();
        // } 
    // }else {
    //     // send failure message to client
    //     let failure = "Server: Username authentication failed".to_string();
    //     control_stream.write(failure.as_bytes()).await.unwrap();
    //     control_stream.flush().await.unwrap();
    //     log::error!("Username authentication failed for user {}", username);
    // }
}

fn create_logger()-> Box<dyn Log>{
    let log_file = std::fs::File::create("server.log").unwrap();
    let config = ConfigBuilder::new()
    .set_time_format_custom(format_description!("[year]:[month]:[day]:[hour]:[minute]:[second].[subsecond]"))
    .set_level_padding(LevelPadding::Left)
    .build();

    let logger = WriteLogger::new(LevelFilter::Info, config, log_file);

    Box::new(logger)
} 

#[tokio::main]
async fn main() {
    
    // Bind to a socket 7878 and start listening for incoming connections.
    let listener = TokioTcpListener::bind("127.0.0.1:7878").await.unwrap();
    // Using mpsc to create channels to handle multiple clients


    // Create the TLS acceptor.
    let mut file = File::open("./certs/my_socket_prog.test.pkcs12.pfx").await.unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).await.unwrap();
    // let cert_password = rpassword::prompt_password("Enter your password to start your server: ").unwrap();
    let cert_password = String::from("Godblessme7787!");
    let cert_password = cert_password.trim().to_string();

    let identity = Identity::from_pkcs12(&identity, &cert_password).unwrap();
    
    let native_acceptor = native_tls::TlsAcceptor::new(identity).unwrap();
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(native_acceptor);

    // Creating logger
    let logging_handle = create_logger();
    let logging_handle: Box<dyn Log> = Box::new(logging_handle);

    log::set_boxed_logger(logging_handle).unwrap();
    log::set_max_level(LevelFilter::Debug);

    let start_message = "Server: Server started successfully!".to_string();

    log::info!("{}", start_message);
    println!("{}", start_message);

    // sharing ownership of tls_acceptor with the spawned task, so that it can be used in the spawned task,
    // ultimately to reduce memory usage.
    // TlsAcceptor is not thread safe, so we need to use Arc to share ownership of it.
    let tls_acceptor = Arc::new(tls_acceptor);

    loop {
        // Asynchronously wait for an inbound socket.
        let (socket, remote_addr) = listener.accept().await.unwrap();

        // Log connection details.
        let connection_msg = format!("Server: Connection established at {} with client from {}!", socket.peer_addr().unwrap(), remote_addr);
        log::info!("{}", connection_msg);
        println!("{}", connection_msg);

        // Create a new TLS acceptor for each communication.
        let tls_acceptor = tls_acceptor.clone();

        tokio::spawn(async move {
            // Accept the TLS connection.
            let tls_stream = tls_acceptor.accept(socket).await.unwrap();
            println!("Server: TLS connection established with client from {}!",  remote_addr);

            handle_control_channel(tls_stream).await;
        });
    }

}

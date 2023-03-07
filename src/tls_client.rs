use std::io;
use native_tls::TlsConnector;

use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::TcpStream
};


// constants
const SERVERADDR: &str = "127.0.0.1:7878";


async fn handle_server(mut tls_stream: tokio_native_tls::TlsStream<TcpStream>) {
    loop {
        // send message to server
        println!("Enter message to send to server: ");
        // read message from the user
        let mut message = String::new();

        io::stdin().read_line(&mut message).unwrap();
        message = message.trim().to_string();
        println!("You entered: {}", message);

        if message.len() == 0 {
            continue;
        }
        // sned message to server
        tls_stream.write(message.as_bytes()).await.unwrap();

        if message.to_lowercase() == "stop" {
            break;
        }

        // read message from server
        let mut buf = [0; 1024];
        let bytes_read = tls_stream.read(&mut buf).await.unwrap();

        let bulk_message =  String::from_utf8_lossy(&buf[..bytes_read]);

        if bulk_message.len() == 0 {
            break;
        }
        println!("Received {} from server", bulk_message);
    }
    println!("Connection to server closed");
}

#[tokio::main]
async fn main() {
    // control transfer
    let tcp_stream = TcpStream::connect(SERVERADDR).await.unwrap();

    // Create a new SSL connector
    let connector = TlsConnector::builder().build().unwrap();
    let connector = tokio_native_tls::TlsConnector::from(connector);

    // Perform the TLS handshake and wrap the stream
    let mut tls_stream = connector.connect("127.0.0.1", tcp_stream).await.unwrap();

    // Read user input of username and password
    let mut credentials = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut credentials).unwrap();
    let username = credentials.trim().to_string();

    let password = rpassword::prompt_password("Enter your password: ").unwrap();
    let password = password.trim().to_string();

    // Combine username and password with a space separator
    let credentials = format!("{} {}", username, password);

    tls_stream.write_all(credentials.as_bytes()).await.unwrap();

    // Read the response from the server, if authentication is successful, then handle the server.
    let mut buf = [0; 512];
    let bytes_read = tls_stream.read(&mut buf).await.unwrap();
    let control_msg = String::from_utf8_lossy(&buf[..bytes_read]);
    
    // Trim the authentication message.
    let control_msg = control_msg.trim().to_string();

    match control_msg.as_str() {
        "Authentication successful!" => {
            handle_server(tls_stream).await;
        },
        _ => println!("Authentication failed."),
    }
}
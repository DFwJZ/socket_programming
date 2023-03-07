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

    handle_server(tls_stream).await;
    // let mut buf = [0; 512];
    // let bytes_read = tls_stream.read(&mut buf).await.unwrap();
    // let control_msg = String::from_utf8_lossy(&buf[..bytes_read]);

    // println!("I am getting {}", control_msg);
    
    // if control_msg == "Authentication successful!" {
    //     handle_server(tls_stream).await;
    // } else {
    //     println!("I am getting {}", control_msg);
    //     println!("Authentication failed.");
    // }
}


// use std::io::{self};
// use std::sync::mpsc::{channel, Sender};

// use native_tls::{TlsConnector};
// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpStream as TokioTcpStream;
// use tokio_native_tls::{TlsConnector as TokioTlsConnector};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a new SSL connector
//     let connector = TlsConnector::new()?;
//     let stream = TokioTcpStream::connect("127.0.0.1:7878").await?;


//     // Read user input of username and password
//     let mut credentials = String::new();
//     println!("Enter your username: ");
//     io::stdin().read_line(&mut credentials)?;
//     let username = credentials.trim().to_string();

//     let password = rpassword::prompt_password("Enter your password: ").unwrap();
//     let password = password.trim().to_string();

//     // Combine username and password with a space separator
//     let credentials = format!("{} {}", username, password);

//     stream.write(credentials.as_bytes()).await?;
//     stream.flush().await?;

//     let (sender, receiver) = channel::<String>();

//     let mut stream = TokioTlsConnector::from(connector).connect("127.0.0.1", stream).await?;

//     // Spawn a separate task to read messages from the server and print them to the console
//     let cloned_stream = stream.clone();
//     tokio::spawn(async move {
//         let mut reader = BufReader::new(cloned_stream);
//         loop {
//             let mut buf = String::new();
//             if reader.read_line(&mut buf).await.is_err() {
//                 break;
//             }
//             sender.send(buf.trim().to_string()).unwrap();
//         }
//     });

//     // Spawn a separate task to read messages from the console and send them to the server
//     tokio::spawn(async move {
//         let mut stdin = io::stdin();
//         loop {
//             let mut buf = String::new();
//             stdin.read_line(&mut buf).unwrap();

//             stream.write_all(buf.as_bytes()).await.unwrap();
//             stream.flush().await.unwrap();

//             if buf.trim() == "stop" {
//                 break;
//             }
//         }
//     });

//     // Read messages from the receiver and print them to the console
//     loop {
//         match receiver.recv() {
//             Ok(msg) => println!("{}", msg),
//             Err(_) => break,
//         }
//     }

//     Ok(())
// }
use std::io::BufRead;

use tokio:: {
    io::{BufReader, AsyncBufReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            
            let mut reader = BufReader::new(reader);
            let mut msg = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut msg) => {
                        if result.is_err() {
                            break;
                        }
                        tx.send((msg.clone(), addr)).unwrap();
                        msg.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr)= result.unwrap();
                        
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }

}
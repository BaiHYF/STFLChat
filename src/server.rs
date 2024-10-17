use tokio::net::{TcpListener, TcpStream};
// use bytes::Bytes;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:7474").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        println!("\x1b[34mSERVER:\x1b[0m accepted connection from: \x1b[32m{}\x1b[0m", addr);
        tokio::spawn(async move {
            let _ = process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0u8; 16];
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        let _ = socket.write(&buffer[..n]).await?;
    }
    anyhow::Ok(())
}
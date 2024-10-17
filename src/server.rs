use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};
use futures::{SinkExt, StreamExt};

const HELP_MSG: &str = "There is no help message yet";

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
    let (reader, writer) = socket.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    
    sink.send(HELP_MSG).await?;

    while let Some(Ok(mut line)) = stream.next().await {
        if line.starts_with("/help") {
            sink.send(HELP_MSG).await?;
        } else if line.starts_with("/exit") {
            break;
        } else {
            line.push_str("😚");
            sink.send(line).await?;
        }
    }
    anyhow::Ok(())
}
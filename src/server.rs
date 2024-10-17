use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::{collections::HashSet, sync::{Arc, Mutex}};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};
use futures::{SinkExt, StreamExt};

const HELP_MSG: &str = "There is no help message yet";
const WELCOM_MSG: &str = "Please enter your username: ";

type UserNames = Arc<Mutex<HashSet<String>>>;

#[tokio::main]
pub async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:7474").await.unwrap();

    let users = UserNames::new(Mutex::new(HashSet::new()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let users = Arc::clone(&users);

        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        println!("\x1b[34mSERVER:\x1b[0m accepted connection from: \x1b[32m{}\x1b[0m", addr);
        tokio::spawn(async move {
            let _ = process_user(socket, users).await;
        });
    }
}

async fn process_user(mut socket: TcpStream, users : UserNames) -> anyhow::Result<()> {
    let (reader, writer) = socket.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    
    sink.send(WELCOM_MSG).await?;
    let mut name = String::new();
    while let Some(Ok(line)) = stream.next().await {
        if !line.is_empty() && !users.lock().unwrap().contains(&line) {
            name = line;
            break;
        } else {
            sink.send("Invalid or occupied username, please try again").await?;
        }
    }
    users.lock().unwrap().insert(name.clone());
    while let Some(Ok(line)) = stream.next().await {
        if line.starts_with("/help") {
            sink.send(HELP_MSG).await?;
        } else if line.starts_with("/exit") {
            break;
        } else if line.starts_with("/users") {
            let users = users.lock().unwrap().clone();
            let mut msg = String::from("\x1b[34mOnline users:\x1b[0m \n");
            for user in users.iter() {
                msg += "   ";
                msg += user;
                msg += "\n ";
            }
            sink.send(msg).await?;
        } 
        else {
            let msgback = format!("From {}: {}", name, line);
            sink.send(msgback).await?;
        }
    }
    anyhow::Ok(())
}
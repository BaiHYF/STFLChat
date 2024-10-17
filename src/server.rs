use tokio::{net::{TcpListener, TcpStream},  sync::broadcast::{self, Sender}};
use std::{collections::HashSet, sync::{Arc, Mutex}};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};
use futures::{SinkExt, StreamExt};
use crate::text::*;

type UserNames = Arc<Mutex<HashSet<String>>>;

#[tokio::main]
pub async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:7474").await.unwrap();

    let users = UserNames::new(Mutex::new(HashSet::new()));
    let (channel_sender, _) = broadcast::channel::<String>(32);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let users = Arc::clone(&users);
        let channel_sender = channel_sender.clone();
        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        println!("\x1b[34mSERVER:\x1b[0m accepted connection from: \x1b[32m{}\x1b[0m", addr);
        tokio::spawn(async move {
            let _ = process_user(socket, users, channel_sender).await;
        });
    }
}

async fn process_user(
    mut socket: TcpStream, 
    users : UserNames,
    channel_sender : Sender<String>,
) -> anyhow::Result<()> {
    let (reader, writer) = socket.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    let mut channel_recevier = channel_sender.subscribe();
    
    sink.send(generate_welcome_message()).await?; // send welcome message

    // Get username input
    sink.send(generate_input_username_message()).await?;
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

    // main loop
    loop {
        sink.send(format!("Hello @{} >", name)).await?;
        tokio::select! {
            send_msg = stream.next() => {
                let send_msg = send_msg.unwrap()?;
                if send_msg.starts_with("/help") {
                    sink.send(generate_help_message()).await?;
                } else if send_msg.starts_with("/exit") {
                    break;
                } else if send_msg.starts_with("/users") {
                    let users = users.lock().unwrap().clone();
                    let mut msg = String::from("\x1b[34mOnline users:\x1b[0m \n");
                    for user in users.iter() {
                        msg += "   ";
                        msg += user;
                        msg += "\n";
                    }
                    sink.send(msg).await?;
                } else if send_msg.starts_with("/send") {
                    let tmp = send_msg.split(" ").collect::<Vec<_>>();
                    if tmp.len() < 3 {
                        sink.send("Usage: /send <username> <message>").await?;
                        continue;
                    }
                    let receiver_user = tmp[1];
                    let send_msg = tmp[2..].join(" ");
                    let msgback = format!("{}|{}|{}", name, receiver_user, send_msg);
                    let _ = channel_sender.send(msgback);
                } else {
                    sink.send(format!("Command not found")).await?;
                }
            },
            recv_msg = channel_recevier.recv() => {        
                let recv_msg = recv_msg?;
                let tmp = recv_msg.split("|").collect::<Vec<_>>();
                let sender = tmp[0];
                let receiver = tmp[1];
                if receiver != name {
                    continue;
                }
                let recv_msg = tmp[2..].join("|");
                sink.send(format!("From {}: {}", sender, recv_msg)).await?;
            }
        }
    }

    users.lock().unwrap().remove(&name);
    anyhow::Ok(())
}


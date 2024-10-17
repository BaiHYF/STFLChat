// use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpStream;
// use tokio::select;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// #[tokio::main]
// async fn main() -> Result<()> {
//     try_main("127.0.0.1:8080").await
// }

// async fn try_main(addr: impl tokio::net::ToSocketAddrs) -> Result<()> {
//     let stream = TcpStream::connect(addr).await?;
//     let (reader, mut writer) = (&stream, &stream);
//     let reader = BufReader::new(reader);
//     let mut lines_from_server = reader.lines();

//     let stdin = BufReader::new(io::stdin());
//     let mut lines_from_stdin = stdin.lines();

//     loop {
//         select! {
//             line = lines_from_server.next_line() => match line {
//                 Ok(Some(line)) => {
//                     println!("{}", line);
//                 },
//                 Ok(None) | Err(_) => break,
//             },
//             line = lines_from_stdin.next_line() => match line {
//                 Ok(Some(line)) => {
//                     writer.write_all(line.as_bytes()).await?;
//                     writer.write_all(b"\n").await?;
//                 },
//                 Ok(None) | Err(_) => break,
//             }
//         }
//     }

//     Ok(())
// }
use std::net::TcpStream as StdTcpStream;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, BufReader};

pub(crate) async fn start_socket(std_stream: StdTcpStream) {
    let Ok(stream) = TcpStream::from_std(std_stream) else { return; };
    println!("Connection established! {:?}", stream);

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    loop {
        let Ok(Some(request_line)) = lines.next_line().await else { return };

        println!("Request: {:?}", request_line);

        loop {
            let Ok(Some(header_line)) = lines.next_line().await else { return };

            if header_line.is_empty() { break; }

            println!("Header: {:?}", header_line);
        }

        println!("Request Done");
    }
}
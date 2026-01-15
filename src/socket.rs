use super::http;

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
        let Some(request_line) = http::decode_request_line(request_line) else { return };

        if request_line.version != "HTTP/1.0" && request_line.version != "HTTP/1.1" { return; }

        println!("Request: {:?}", request_line);

        loop {
            let Ok(Some(header)) = lines.next_line().await else { return };
            if header.is_empty() { break; }

            let Some(header) = http::decode_header(header) else { return };

            println!("Header: {:?}", header);
        }

        println!("Request Done");
    }
}
mod socket;

use std::thread::available_parallelism;
use std::net::TcpListener;
use tokio::runtime::Builder;

fn main() {
  let runtime = Builder::new_multi_thread()
      .worker_threads(available_parallelism().unwrap().get())
      .enable_io()
      .build()
      .unwrap();

  let rt_handle = runtime.handle();

  let listener = TcpListener::bind("127.0.0.1:6000").unwrap();

  for stream in listener.incoming() {
    let stream = match stream {
      Ok(stream) => stream,
      Err(err) => {
        println!("Accept error: {}", err);
        continue;
      }
    };

    if stream.set_nonblocking(true).is_err() {
      println!("Failed to set nonblocking");
      continue;
    }

    rt_handle.spawn(async move {
      socket::start_socket(stream).await;
    });
  }
}

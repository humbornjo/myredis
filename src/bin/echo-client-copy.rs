use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    let (mut rd, mut wr) = io::split(socket);

    let t1 = tokio::spawn(async move {
        loop {
            wr.write_all(b"hello\r\n").await.unwrap();
            wr.write_all(b"world\r\n").await.unwrap();

            sleep(Duration::from_millis(1000)).await;
        }
    });

    let mut buf = vec![0; 128];
    let t2 = tokio::spawn(async move {
        loop {
            let rep = rd.read(&mut buf).await.unwrap();
            if rep == 0 {
                break;
            }
            println!("GOT {:?}", str::from_utf8(&buf[..rep]).unwrap());
        }
    });
    let _ = t1.await;
    let _ = t2.await;
}

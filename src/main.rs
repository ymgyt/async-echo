use std::error::Error;
use tokio::net;
use tokio::prelude::*;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:6666";
    println!("bind {}", addr);
    let mut listener = net::TcpListener::bind(addr).await?;

    loop {
        let (stream, remote) = listener.accept().await?;
        task::spawn(async move {
            println!("accept {}", remote);
            match handle_conn(stream).await {
                Ok(_) => println!("close {}", remote),
                Err(err) => eprintln!("{} error: {}", remote, err),
            }
        });
    }
}

async fn handle_conn(mut conn: net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buff = [0u8; 1024];
    loop {
        let n = conn.read(&mut buff).await?;
        println!("read {} bytes", n);
        if n == 0 {
            break
        }
        println!("write {}", String::from_utf8_lossy(&buff[0..n]));
        conn.write(&mut buff[0..n]).await?;
    }
    Ok(())
}

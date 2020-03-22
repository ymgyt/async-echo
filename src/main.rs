use std::error::Error;
use tokio::{prelude::*, net,task};
use tracing::{info,trace, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    let addr = "127.0.0.1:6666";
    info!("bind {}", addr);
    let mut listener = net::TcpListener::bind(addr).await?;

    loop {
        let (stream, remote) = listener.accept().await?;
        task::spawn(async move {
            info!("accept {}", remote);
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
        trace!("read {} bytes", n);
        if n == 0 {
            break
        }
        trace!("write {}", String::from_utf8_lossy(&buff[0..n]));
        conn.write(&mut buff[0..n]).await?;
    }
    Ok(())
}

fn init_logger() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting subscriber failed");
}

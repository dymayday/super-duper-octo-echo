//! The reveiver part : the only logic here is to echo what we received.

#[macro_use]
extern crate log;
use cio::message::Payload;
use env_logger;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7788".into());

    env_logger::builder().try_init()?;

    let listener = TcpListener::bind(&addr).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let mut buf = [0; 1024];
                let mut m = 0;
                'read_loop: loop {
                    stream.readable().await?;
                    match stream.try_read(&mut buf[..]) {
                        Ok(0) => break 'read_loop,
                        Ok(n) => {
                            m = n;
                            break 'read_loop;
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }

                match Payload::from_bytes(&buf[..m]) {
                    Ok(payload) => {
                        info!("Got : {:?}", payload);
                    }
                    Err(err) => {
                        error!("Error during `Payload` parsing: {}", err);
                    }
                }
            }
            Err(err) => {
                warn!("Listerner failed to accept connection : {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

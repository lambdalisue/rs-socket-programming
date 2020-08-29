use anyhow::Result;
use async_std::io;
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::prelude::*;
use async_std::task;
use log::{debug, error};
use std::env;

async fn serve(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        debug!("Accepting from: {}", stream.peer_addr()?);
        let _handle = task::spawn(handler(stream));
    }
    Ok(())
}

async fn handler(stream: TcpStream) -> Result<()> {
    let mut reader = stream.clone();
    let mut writer = stream;
    io::copy(&mut reader, &mut writer).await?;
    Ok(())
}

#[async_std::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify [addr:port].");
        std::process::exit(1);
    }
    let addr = &args[1];
    serve(addr).await.unwrap_or_else(|e| error!("{}", e));
}

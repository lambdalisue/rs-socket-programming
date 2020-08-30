use anyhow::Result;
use async_std::net::{ToSocketAddrs, UdpSocket};
use log::error;
use std::env;

async fn serve(addr: impl ToSocketAddrs) -> Result<()> {
    let socket = UdpSocket::bind(addr).await?;
    loop {
        let mut buf = [0u8; 1024];
        let (_, peer) = socket.recv_from(&mut buf).await?;
        socket.send_to(&buf, peer).await?;
    }
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

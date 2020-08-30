use anyhow::Result;
use async_std::io;
use async_std::net::{ToSocketAddrs, UdpSocket};
use log::error;
use std::env;
use std::str;

async fn communicate(addr: impl ToSocketAddrs) -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).await?;
        socket.send_to(input.as_bytes(), &addr).await?;

        let mut buf = [0u8; 1024];
        socket.recv_from(&mut buf).await?;
        print!("{}", str::from_utf8(&buf)?);
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
    communicate(addr).await.unwrap_or_else(|e| error!("{}", e));
}

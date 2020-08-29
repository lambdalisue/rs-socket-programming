use anyhow::Result;
use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use futures::try_join;
use log::error;
use std::env;

async fn connect(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let mut reader1 = io::stdin();
    let mut reader2 = stream.clone();
    let mut writer1 = stream;
    let mut writer2 = io::stdout();
    try_join!(
        io::copy(&mut reader1, &mut writer1),
        io::copy(&mut reader2, &mut writer2),
    )?;
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
    connect(addr).await.unwrap_or_else(|e| error!("{}", e));
}

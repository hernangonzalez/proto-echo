use anyhow::{Ok, Result};
use futures::SinkExt;
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinSet,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Echo is listening to connections...");

    let mut handles = JoinSet::new();
    while let core::result::Result::Ok(res) = listener.accept().await {
        println!("New connection from: {}", res.1);
        let stream = res.0;
        handles.spawn(async move { work_connection(stream).await });
    }

    while (handles.join_next().await).is_some() {}

    Ok(())
}

async fn work_connection(stream: TcpStream) -> Result<()> {
    let mut framed = Framed::new(stream, BytesCodec::new());
    while let Some(message) = framed.next().await {
        let bytes = message?;
        println!("Message: {}", String::from_utf8_lossy(&bytes));
        framed.send(bytes).await?;
    }

    println!(
        "No more frames from: {}",
        framed.into_inner().peer_addr().unwrap()
    );
    Ok(())
}

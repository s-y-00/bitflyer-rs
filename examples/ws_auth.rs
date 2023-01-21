use bitflyer::websocket::{BitFlyerWebsocket, Command, Topic};
use chrono::{Duration, Utc};
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut client =
        BitFlyerWebsocket::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?).await?;

    println!("WebSocket handshake has been successfully completed");
    let expires = (Utc::now() + Duration::seconds(30)).timestamp();

    client.send(Command::authenticate(expires as u64)).await?;

    client
        .send(Command::Subscribe(vec![Topic::Position]))
        .await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}

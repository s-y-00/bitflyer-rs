use bitflyer::websocket::{BitFlyerWebsocket, Command, Topic};
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut client =
        BitFlyerWebsocket::with_credential(&var("BITFLYER_KEY")?, &var("BITFLYER_SECRET")?).await?;

    println!("WebSocket handshake has been successfully completed");

    client
        .send(Command::Subscribe(vec![
            Topic::BoardSnapshot(Some("BTC_JPY".to_string())),
            Topic::Board(Some("BTC_JPY".to_string())),
            Topic::Ticker(Some("BTC_JPY".to_string())),
        ]))
        .await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}

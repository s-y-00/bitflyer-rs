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
        BitFlyerWebsocket::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?).await?;

    println!("WebSocket handshake has been successfully completed");

    client
        .send(Command::Subscribe(vec![
            // Topic::OrderBookL2_25(Some("XBTUSD".to_string())),
            Topic::Trade(Some("XBTUSD".to_string())),
        ]))
        .await?;

    while let Some(msg) = client.next().await {
        println!("{:?}", msg);
    }

    Ok(())
}

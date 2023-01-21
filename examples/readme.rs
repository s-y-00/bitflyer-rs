use chrono::{Duration, Utc};
use failure::Fallible;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    // This will give you a BitFlyer instance, which the only purpose is to create connection.
    let bm = bitflyer::rest::BitFlyerRest::with_credential(
        &std::env::var("BITFLYER_KEY")?,
        &std::env::var("BITMEX_SECRET")?,
    );

    // All the requests to BitFlyer server afterwards will go through HTTP Restful API.

    // The request models reside in "bitflyer::models" module, with the
    // naming convention of "Method+camelCase(endpoint)+Request", e.g. "GET /trade/bucketed" would be
    // "bitflyer::models::GetTradeBucketedRequest" in bitmex-rs.
    let req = bitflyer::rest::GetTradeBucketedRequest {
        bin_size: Some(bitflyer::rest::BinSize::D1),
        ..Default::default()
    };

    // Request to BitFlyer server is made by giving "BitFlyer::request" the request object.
    // The return type of "BitFlyer::request" is a future of the response so that you can await on it.
    let resp = bm.request(req).await?;
    println!("Bucketed trades: {:?}", resp);

    // A websocket is created by "BitFlyer::websocket".
    let mut ws = bitflyer::websocket::BitFlyerWebsocket::with_credential(
        &std::env::var("BITFLYER_KEY")?,
        &std::env::var("BITMEX_SECRET")?,
    )
    .await?;

    // The websocket is a duplex channel which means you can send "bitflyer::websocket::Command" to BitFlyer and
    // receive "bitflyer::websocket::Message" from BitFlyer using it.
    let expires = (Utc::now() + Duration::seconds(30)).timestamp();
    ws.send(bitflyer::websocket::Command::authenticate(expires as u64))
        .await?;

    // In order to get the ws messages, just poll the ws stream.
    while let Some(message) = ws.next().await {
        println!("Subscription message received {:?}", message);
    }

    Ok(())
}

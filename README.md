# bitmex-rs [![Crates.io](https://img.shields.io/crates/v/bitmex.svg)](https://crates.io/crates/bitmex) [![Build Status](https://travis-ci.org/dovahcrow/bitmex-rs.png?branch=master)](https://travis-ci.org/dovahcrow/bitmex-rs) [![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

[BitFlyer](https://www.bitmex.com/app/apiOverview) (non-official) client for rust, with async/await support!

[Documentation](https://docs.rs/crate/bitmex)

## Caveat

Please run the tests before you use since BitFlyer often introduces breaking changes without
changing their Swagger API definition.

Also, the Swagger definition somehow is not aligned with BitFlyer's realworld API. Since bitmex-rs
auto-generates the code from swagger.json, you may experience some API breakage. Please do not 
hesitate to file an issue!

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
bitmex = "0.2"
```

# Basic usage

```rust
// This will give you a BitFlyer instance, which the only purpose is to create connection.
let bm = bitflyer::BitFlyerRest::with_credential(&std::env::var("BITFLYER_KEY")?, &std::env::var("BITMEX_SECRET")?);

// All the requests to BitFlyer server afterwards will go through HTTP Restful API.

// The request models reside in "bitflyer::models" module, with the
// naming convention of "Method+camelCase(endpoint)+Request", e.g. "GET /trade/bucketed" would be
// "bitflyer::models::GetTradeBucketedRequest" in bitmex-rs.
let req = bitflyer::models::GetTradeBucketedRequest {
    bin_size: Some(bitflyer::models::BinSize::D1),
    ..Default::default()
};

// Request to BitFlyer server is made by giving "BitFlyer::request" the request object.
// The return type of "BitFlyer::request" is a future of the response so that you can await on it.
let resp = bm.request(req).await?;
println!("Bucketed trades: {:?}", resp);  

// A websocket is created by "BitFlyer::websocket".
let mut ws = bm.websocket().await?;

// The websocket is a duplex channel which means you can send "bitflyer::websocket::Command" to BitFlyer and 
// receive "bitflyer::websocket::Message" from BitFlyer using it.
let expires = (Utc::now() + Duration::seconds(30)).timestamp();
ws.send(Command::authenticate(&bm, expires).unwrap()).await?;

// In order to get the ws messages, just poll the ws stream.
while let Some(message) = ws.next().await {
    println!("Subscription message received {:?}", message);
}

```

More examples located in the examples and tests folder.

## Implementation status

Currently all the API features are implemented, including websocket! 

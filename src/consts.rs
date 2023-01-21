use std::env::var;

use lazy_static::lazy_static;
use log::warn;

// dotenv is a must run in every test otherwise the url will be mis-loaded
lazy_static! {
    pub static ref WS_URL: &'static str = {
        if var("BITMEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "wss://ws.lightstream.bitflyer.com/json-rpc"
        } else {
            warn!("Your are using BitFlyer testnet Websocket");
            "wss://testnet.bitmex.com/realtime"
        }
    };
    pub static ref REST_URL: &'static str = {
        if var("BITMEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "https://api.bitflyer.com/v1/"
        } else {
            warn!("Your are using BitFlyer testnet Restful API");
            "https://testnet.bitmex.com/api/v1"
        }
    };
}

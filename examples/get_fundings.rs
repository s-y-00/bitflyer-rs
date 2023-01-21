use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::GetFundingRequest;
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let bm = BitFlyerRest::new();
    let res = bm
        .request(GetFundingRequest {
            symbol: Some("XBT".to_string()),
            ..Default::default()
        })
        .await?;

    println!("{:?}", res);
    Ok(())
}

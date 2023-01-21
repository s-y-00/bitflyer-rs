use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::GetSettlementRequest;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_settlement() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetSettlementRequest {
        ..Default::default()
    });
    let ret = rt.block_on(fut);

    debug!("{:?}", ret);
    Ok(())
}

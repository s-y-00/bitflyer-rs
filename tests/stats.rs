use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::{GetStatsHistoryRequest, GetStatsHistoryUSDRequest, GetStatsRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_stats() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetStatsRequest);
    let ret = rt.block_on(fut);

    debug!("{:?}", ret);
    Ok(())
}

#[test]
fn get_stats_history() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetStatsHistoryRequest);
    let ret = rt.block_on(fut);

    debug!("{:?}", ret);
    Ok(())
}

#[test]
fn get_stats_history_usd() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetStatsHistoryUSDRequest);
    let ret = rt.block_on(fut);

    debug!("{:?}", ret);
    Ok(())
}

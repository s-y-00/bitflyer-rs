use bitflyer::rest::BinSize;
use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::{GetQuoteBucketedRequest, GetQuoteRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

// 403 forbidden
#[test]
fn get_quote() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetQuoteRequest {
        ..Default::default()
    });
    let ret = rt.block_on(fut);
    debug!("{:?}", ret);

    assert!(ret.is_err());
    Ok(())
}

// 403 forbidden
#[test]
fn get_quote_bucketed() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetQuoteBucketedRequest {
        partial: Some(false),
        bin_size: Some(BinSize::D1),
        ..Default::default()
    });
    let ret = rt.block_on(fut);
    debug!("{:?}", ret);

    assert!(ret.is_err());
    Ok(())
}

use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::{
    GetPositionRequest, PostPositionIsolateRequest, PostPositionLeverageRequest,
    PostPositionRiskLimitRequest, PostPositionTransferMarginRequest,
};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_position() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let _ = ::env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetPositionRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn post_position_isolate() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let _ = ::env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(PostPositionIsolateRequest {
        symbol: "XBTUSD".into(),
        enabled: Some(false),
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn post_position_leverage() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let _ = ::env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(PostPositionLeverageRequest {
        symbol: "XBTUSD".into(),
        leverage: 1.1,
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn post_position_risk_limit() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let _ = ::env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(PostPositionRiskLimitRequest {
        symbol: "XBTUSD".into(),
        risk_limit: 30000000000,
    });

    let a = rt.block_on(fut)?;
    println!("{:?}", a);
    Ok(())
}

#[test]
#[ignore]
fn post_position_transfer_margin() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let _ = ::env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(PostPositionTransferMarginRequest {
        symbol: "XBTUSD".into(),
        amount: 10,
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

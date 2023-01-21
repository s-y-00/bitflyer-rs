use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::GetOrderBookL2Request;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_order_book_l2() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetOrderBookL2Request {
        symbol: "XBTUSD".into(),
        depth: Some(1),
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

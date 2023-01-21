use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::{GetAnnouncementRequest, GetAnnouncementUrgentRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_announcement() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let rt = Runtime::new()?;

    let bm = BitFlyerRest::new();
    let fut = bm.request(GetAnnouncementRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_announcement_urgent() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetAnnouncementUrgentRequest {});

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

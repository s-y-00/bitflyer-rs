use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::GetUserEventRequest;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_user_event() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let rt = Runtime::new()?;
    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);

    let _ = rt.block_on(bm.request(GetUserEventRequest::default()))?;
    Ok(())
}

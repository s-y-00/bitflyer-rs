use bitflyer::rest::BitFlyerRest;
use bitflyer::rest::GetApiKeyRequest;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_api_key() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let rt = Runtime::new()?;

    let bm = BitFlyerRest::with_credential(&var("BITFLYER_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetApiKeyRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

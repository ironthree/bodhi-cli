use bodhi::BodhiService;
use bodhi::Override;
use bodhi::OverrideNVRQuery;
use bodhi::Update;
use bodhi::UpdateIDQuery;

pub async fn query_override(bodhi: &BodhiService, nvr: &str) -> Result<Override, String> {
    bodhi
        .request(&OverrideNVRQuery::new(nvr))
        .await
        .map_err(|error| error.to_string())
}

pub async fn query_update(bodhi: &BodhiService, alias: &str) -> Result<Update, String> {
    bodhi
        .request(&UpdateIDQuery::new(alias))
        .await
        .map_err(|error| error.to_string())
}

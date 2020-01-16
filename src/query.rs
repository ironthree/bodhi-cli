use bodhi::BodhiService;
use bodhi::Override;
use bodhi::OverrideNVRQuery;
use bodhi::Update;
use bodhi::UpdateIDQuery;

pub fn query_override(bodhi: &BodhiService, nvr: &str) -> Result<Override, String> {
    match bodhi.query(OverrideNVRQuery::new(&nvr)) {
        Ok(value) => match value {
            Some(over_ride) => Ok(over_ride),
            None => Err(format!("No override found for NVR: {}", nvr)),
        },
        Err(error) => Err(error.to_string()),
    }
}

pub fn query_update(bodhi: &BodhiService, alias: &str) -> Result<Update, String> {
    match bodhi.query(UpdateIDQuery::new(&alias)) {
        Ok(value) => match value {
            Some(update) => Ok(update),
            None => Err(format!("No update found with this alias: {}", alias)),
        },
        Err(error) => Err(error.to_string()),
    }
}

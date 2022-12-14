use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logo {
    pub url: String,
    pub url_org_logo_m: String,
}

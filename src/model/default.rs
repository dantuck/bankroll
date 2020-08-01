use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Default {
    pub account: Option<String>,
    pub account_offset: Option<String>,
}

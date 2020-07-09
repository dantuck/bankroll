use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Default {
    pub account: String,
    pub account_offset: String,
}
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Default {
    pub account: String,
    pub account_offset: String,
}
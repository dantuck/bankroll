use ansi_term::Colour;
use serde::Deserialize;
use std::collections::HashMap;
use std::default::Default as StdDfault;

// use ansi_term::Colour::{RGB, Green, Red, Yellow, White};
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ThresholdOperator {
    LessThan,
    LessThanOrEqual,
    Equal,
    MoreThanOrEqual,
    MoreThan,
    Between(f64, f64),
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ThresholdColor {
    Green,
    Red,
    Yellow,
    White,
    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

impl ThresholdColor {
    pub fn to_ansi_color(&self) -> Colour {
        match self {
            ThresholdColor::Green => Colour::Green,
            ThresholdColor::Red => Colour::Red,
            ThresholdColor::Yellow => Colour::Yellow,
            ThresholdColor::White => Colour::White,
            ThresholdColor::RGB(a, b, c) => Colour::RGB(*a, *b, *c),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum AccountThreshold {
    Limit {
        limit: f64,
        color: ThresholdColor,
        operator: ThresholdOperator,
    },
}

#[derive(Debug, Clone)]
pub struct AccountSetting {
    pub account_thresholds: Vec<AccountThreshold>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Setting {
    pub account_name: String,
    pub account_threshold: Option<Vec<AccountThreshold>>,
}

#[derive(Deserialize, Debug, Clone, StdDfault)]
pub struct Default {
    pub account: Option<String>,
    pub account_offset: Option<String>,
    pub setting: Option<Vec<Setting>>,
}

impl Default {
    pub fn get_account_settings(&self) -> HashMap<String, AccountSetting> {
        let mut account_settings: HashMap<String, AccountSetting> = HashMap::new();

        if let Some(settings) = &self.setting {
            for setting in settings {
                match account_settings.get_mut(&setting.account_name) {
                    Some(acc_setting) => {
                        if let Some(account_threshold) = &setting.account_threshold {
                            acc_setting
                                .account_thresholds
                                .copy_from_slice(&account_threshold);
                        }
                    }
                    None => {
                        account_settings.insert(
                            setting.account_name.to_string(),
                            AccountSetting {
                                account_thresholds: setting
                                    .account_threshold
                                    .as_ref()
                                    .unwrap_or(&Vec::new())
                                    .to_vec(),
                            },
                        );
                    }
                }
            }
        }

        account_settings
    }
}

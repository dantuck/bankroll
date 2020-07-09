#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub balance: f64,
}

#[macro_export]
macro_rules! account {
    ($x:expr, $y:expr) => {
        Account {
            name: $x.to_string(),
            balance: $y,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account() {
        let account = Account {
            name: "test account".to_string(),
            balance: 20.00,
        };

        let test_account = account!("test account", 20.00);
        assert_eq!(test_account.name, account.name);
        assert_eq!(test_account.balance, account.balance);
    }
}
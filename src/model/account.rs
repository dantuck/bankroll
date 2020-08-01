#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub balance: f64,
}

pub fn fitaccount(name: &String, chars: usize) -> String {
    if chars > name.len() {
        return name.to_string();
    }

    let account_vec: Vec<&str> = name.split(":").collect();
    let mut result: String = "".to_string();

    if account_vec.len() > 1 {
        if let Some((first, elements)) = account_vec.split_first() {
            let first_len = first.len();
            if let Some((last, elements)) = elements.split_last() {
                let last_len = last.len();
                let mut length = name.len().wrapping_sub(chars);
                let trim_at: usize = if chars <= 10 {
                    if last_len >= 3 {
                        2
                    } else {
                        3
                    }
                } else {
                    let (sub, is_overflow) = first_len.overflowing_sub(length);
                    if !is_overflow && sub > 3 {
                        sub
                    } else {
                        first_len
                    }
                };

                if first_len > trim_at {
                    let (keep, c) = first.split_at(trim_at);
                    result.push_str(keep);
                    length -= c.len();
                } else {
                    result.push_str(first);
                }

                for e in elements {
                    result.push_str(":");

                    if e.len() > 3 {
                        let (sub, is_overflow) = e.len().overflowing_sub(length);

                        let split_at: usize;

                        if is_overflow || sub <= 3 {
                            split_at = 3;
                        } else {
                            split_at = sub;
                        }

                        let (keep, _) = e.split_at(split_at);
                        result.push_str(keep);
                        let (sub, is_overflow) = e.len().overflowing_sub(split_at);
                        if !is_overflow && length > 0 {
                            length -= sub
                        }
                    } else {
                        result.push_str(e);
                    }
                }

                if let Some(sub) = last_len.checked_sub(length) {
                    length = sub
                }

                let (keep, _) = last.split_at(length);
                result.push_str(":");
                result.push_str(keep);
            }
        }
    }

    result
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

    #[test]
    fn test_fit_accountname_to() {
        let name = "Asset:Name:One".to_string();
        let shortened1 = fitaccount(&name, 10);
        assert_eq!(shortened1, "As:Nam:One");
        assert_eq!(shortened1.len(), 10);

        let name = "Asset:Name:OneAndSome".to_string();
        let shortened = fitaccount(&name, 10);
        assert_eq!(shortened, "As:Nam:One");
        assert_eq!(shortened.len(), 10);

        let name = "Asset:Name:On".to_string();
        let shortened = fitaccount(&name, 10);
        assert_eq!(shortened, "Ass:Nam:On");
        assert_eq!(shortened.len(), 10);

        let name = "Expenses:general".to_string();
        let shortened = fitaccount(&name, 20);
        assert_eq!(shortened, "Expenses:general");
        assert_eq!(shortened.len(), 16);

        let name = "Equity:opening balance".to_string();
        let shortened = fitaccount(&name, 20);
        assert_eq!(shortened, "Equi:opening balance");
        assert_eq!(shortened.len(), 20);

        // let name = "Asset:Name:On".to_string();
        // let shortened = name.fit_accountname_to(5);
        // assert_eq!(shortened, "Ass:Nam:On");
        // assert_eq!(shortened.len(), 10);
    }
}

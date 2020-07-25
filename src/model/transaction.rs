use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::{account};

pub trait Truncate {
    fn fit_to(&self, chars: usize) -> String;
    fn truncate_to_offset(&self, boundary: usize) -> String;
}

impl Truncate for str {
    fn fit_to(&self, chars: usize) -> String {
        if chars == 0 {
            return self[..0].to_string();
        }

        match self.char_indices().nth(chars) {
            None => return self.to_string(),
            Some((boundary, _)) => {
                return self.truncate_to_offset(boundary);
            }
        };
    }

    fn truncate_to_offset(&self, boundary: usize) -> String {
        if boundary > self.len() {
            return self.to_string()
        }

        let mut char_iter = self
            .char_indices()
            .rev()
            .skip_while(move |(n, char)| {
                *n > boundary - 2 || !char.is_ascii_whitespace()
            });

        let mut charcount = boundary;

        if let Some((bound, _)) = char_iter.next() {
            charcount = bound;
        };

        let mut truncated: String = self[..charcount].trim_end().to_string();

        truncated.push_str("…");

        truncated
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub amount: f64,
    pub account: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub account: Option<String>,
    pub account_offset: Option<String>,
    pub amount: Option<f64>,
    pub post: Option<Vec<Post>>,
}

#[derive(Clone, Debug)]
pub struct TransactionMeta {
    pub date: String,
    pub description: String,
    pub posts: Vec<Post>,
    pub accounts: HashMap<String, account::Account>,
}

impl Transaction {
    pub fn parse(self) -> TransactionMeta {
        let mut check: f64 = 0.0;
        let mut accounts: HashMap<String, account::Account> = HashMap::new();
        let mut posts_parsed: Vec<Post> = Vec::new();

        if let Some(posts) = self.post {
            for post in posts {
                check += post.amount;
                posts_parsed.push(post);
            }
        }
        else if let Some(_t) = self.amount {
            let offset_account = &self.account_offset;
            let post = Post {
                amount: self.amount.unwrap(),
                account: offset_account.as_ref().unwrap().to_string()
            };

            check += post.amount;
            posts_parsed.push(post);
        }

        if check != 0.0 {
            if let Some(account) = self.account {
                let check_inverse = if check > 0.0 {
                    -check 
                } else { 
                    check.abs() 
                };
                
                posts_parsed.push(Post {
                    amount: check_inverse,
                    account: account,
                });
            }
        }

        for post in &posts_parsed {
            match accounts.get_mut(&post.account) {
                Some(account) => {
                    account.balance += &post.amount;
                },
                None => {
                    accounts.insert(
                        post.account.to_string(),
                        account::Account {
                            name: post.account.to_string(),
                            balance: post.amount,
                        }
                    );
                }
            }
        }

        TransactionMeta {
            date: self.date,
            description: self.description,
            posts: posts_parsed,
            accounts: accounts,
        }
    }
}

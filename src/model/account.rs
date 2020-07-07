// use std::ops::{AddAssign};

#[derive(Clone)]
pub struct Account {
    pub name: String,
    pub balance: f64,
}

// pub struct Accounts {

// }

// pub struct AccountBalance {
//     pub amount: f64
// }

// impl AddAssign for AccountBalance {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             amount: self.amount + other.amount,
//         };
//     }
// }
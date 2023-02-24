use std::{
    thread::sleep,
    time::Duration,
};
use serde::{Serialize, Deserialize};

pub mod slots;

#[derive(Serialize, Deserialize)]
pub struct AccConfig {
    init_balance: isize,
    init_bet: usize,
    bet_inc: usize,
}

impl std::default::Default for AccConfig {
    fn default() -> Self {
        Self {
            init_balance: 100,
            init_bet: 5,
            bet_inc: 1,
        }
    }
}

pub struct Account {
    balance: isize,
    bet: usize,
    inc: usize,
}

pub enum PL {
    Profit(usize),
    Loss,
}

impl Account {
    pub fn new(conf: &AccConfig) -> Self {
        Account {
            balance: conf.init_balance,
            bet: conf.init_bet,
            inc: conf.bet_inc,
        }
    }
}

pub fn e_bet(bet: &usize, inc: &usize, side: bool) -> usize {
    if side {
        bet.checked_add(*inc).unwrap_or(usize::MAX)
    } else {
        bet.checked_sub(*inc).unwrap_or(usize::MIN)
    }
}

fn e_bal(bet: &usize, balance: &isize, hit: PL) -> (isize, String) {
    match hit {
        // Most gamblers quit right before they're about to hit it big
        PL::Profit(multi) => {
            let win = bet * multi;
            (balance.checked_add_unsigned(win).unwrap_or(isize::MAX),
            format!("You win {win}!"))
        }
        PL::Loss => {
            (balance.checked_sub_unsigned(*bet).unwrap_or(isize::MIN),
            format!("Broke ass"))
        }
    }
}

// Convenience
pub fn pause() {
    sleep(Duration::from_secs(2))
}


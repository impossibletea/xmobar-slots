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

    pub fn e_bet(&mut self, side: bool) {
        self.bet = if side {
            self.bet
                .checked_add(self.inc)
                .unwrap_or(usize::MAX)
        } else {
            self.bet
                .checked_sub(self.inc)
                .unwrap_or(usize::MIN)
        };
        println!("Bet: {}", self.bet);
        pause();
    }

    fn e_bal(&mut self, hit: PL) {
        match hit {
            // Most gamblers quit right before they're about to hit it big
            PL::Profit(multi) => {
                let win = self.bet * multi;
                self.balance = self.balance
                    .checked_add_unsigned(win)
                    .unwrap_or(isize::MAX);
                println!("You win {win}!")
            }
            PL::Loss => {
                self.balance = self.balance
                    .checked_sub_unsigned(self.bet)
                    .unwrap_or(isize::MIN);
                println!("Broke ass")
            }
        }
        pause();
    }
}

// Convenience
pub fn pause() {
    sleep(Duration::from_secs(2))
}


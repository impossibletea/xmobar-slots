use std::{
    thread::sleep,
    time::Duration,
};
use std::fmt::Display;
use serde::{Serialize, Deserialize};

pub mod slots;

#[derive(Serialize, Deserialize)]
pub struct AccConfig {
    init_balance: isize,
    init_bet:     usize,
    bet_inc:      usize,
}

impl std::default::Default for AccConfig {
    fn default() -> Self {
        Self {
            init_balance: 100,
            init_bet:     5,
            bet_inc:      1,
        }
    }
}

pub struct Account {
    pub balance: isize,
    pub bet:     usize,
    inc:     usize,
}

impl Account {
    pub fn new(conf: &AccConfig) -> Self {
        Account {
            balance: conf.init_balance,
            bet:     conf.init_bet,
            inc:     conf.bet_inc,
        }
    }

    pub fn e_bet(&mut self, side: bool) {
        self.bet = if side {
            self.bet.saturating_add(self.inc)
        } else {
            self.bet.saturating_sub(self.inc)
        }
    }

    pub fn e_bal(&mut self, hit: PL) {
        let bet = self.bet;
        self.balance = match hit {
            // Most gamblers quit right before they're about to hit it big
            PL::Profit(multi) => {
                let win = bet * multi;
                println!("You win {win}!");
                self.balance.saturating_add_unsigned(win)
            }
            PL::Loss => {
                println!("Broke ass");
                self.balance.saturating_sub_unsigned(bet)
            }
        };
    }
}

pub trait Controls {
    fn play (&self, sig: i32) -> Option<PL>;
}

pub struct Game<T: Controls + Display> {
    pub id:   String,
    pub name: String,
    pub game: T,
}

impl<T: Controls + Display> Controls for Game<T> {
    fn play (&self, sig: i32) -> Option<PL> {
        self.game.play(sig)
    }
}

impl<T: Controls + Display> Display for Game<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub enum PL {
    Profit(usize),
    Loss,
}

// Convenience
pub fn pause() {
    sleep(Duration::from_secs(2))
}


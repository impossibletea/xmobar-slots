use std::{
    thread::sleep,
    time::Duration,
    fmt::Display,
};
use crate::Status;
use signal_hook::consts::signal::*;
use serde::{Serialize, Deserialize};

pub mod slots;
pub mod roulette;

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
        pause();
    }

    pub fn signal(&mut self, sig: i32) -> Status {
        match sig {
            SIGUSR1 => self.e_bet(false),
            SIGUSR2 => self.e_bet(true),
            _       => return Status::Selecting,
        }
        Status::Balancing
    }
}

pub trait Controls {
    fn play (&mut self, sig: i32) -> Loop;
}

pub trait Playable: Controls + Display {
    fn name(&self) -> String {
        "Name your shit".to_string()
    }
}

pub type Game = Box<dyn Playable>;

pub enum Loop {
    InGame(Option<PL>),
    Balance,
    Exit
}

pub enum PL {
    Profit(usize),
    Loss,
}

// Convenience
pub fn pause() {
    sleep(Duration::from_secs(2))
}


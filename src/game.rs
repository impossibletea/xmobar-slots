use std::{
    thread::sleep,
    time::Duration,
    collections::HashMap,
};
use crate::slots::Slots;
use crate::Config;

// Could just return a number, but this makes it more obvious
pub enum PL {
    Profit(usize),
    Loss,
}

pub struct Account {
    pub balance: isize,
    pub bet: usize,
    inc: usize,
}

impl Account {
    pub fn new(conf: Config) -> Self {
        Account {
            // Config will affect later
            balance: 1000,
            bet: 5,
            inc: 5,
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
        }
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
    }
}

pub fn home(machine: &Slots, bet: &usize) {
    println!("{machine} Bet: {bet}")
}

pub fn round(player: &mut Account, machine: &mut Slots) -> () {
    // Roll the dice
    machine.roll();
    pause();

    // Calculate profit
    let result = combination_check(&machine.to_string());
    // Change balance accordingly
    player.e_bal(result);
    pause();

    // Display starter screen
    home(machine, &player.bet)
}

// Convenience
pub fn pause() {
    sleep(Duration::from_secs(2))
}

// The most basic I've come up with, probably should represent real slots more
fn combination_check(game: &String) -> PL {
    let mut totals = HashMap::<char, usize>::new();
    for i in game.chars() {
        totals.entry(i).and_modify(|n| {*n += 1}).or_insert(0);
    }

    // If I fucked up here, you get no money
    let multiplier = match totals.values().max() {
        Some(k) => *k,
        None => 0,
    };

    if multiplier == 0 {PL::Loss} else {PL::Profit(multiplier)}
}



use std::{process, fmt::Display};
use serde::{Serialize, Deserialize};
use signal_hook::{consts::signal::*, iterator::Signals};

mod games;
use games::{Game, Controls, Account, Loop};

#[derive(Serialize, Deserialize)]
pub struct Config {
    account: games::AccConfig,
    slots:   games::slots::SlotConfig,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            account: games::AccConfig::default(),
            slots:   games::slots::SlotConfig::default(),
        }
    }
}

fn main() {
    let inputs = [
        SIGCONT, // Enter
        SIGUSR1, // Scroll up
        SIGUSR2, // Scroll down
        SIGINT,  // Esc
    ];
    let mut signals = match Signals::new(&inputs) {
        Ok(result) => result,
        Err(err)   => {
            eprintln!("{err}");
            process::exit(1)
        }
    };

    let conf = confy::load("xmobet", "config").unwrap_or(Config::default());
    let mut account = Account::new(&conf.account);
    let mut game_list = [
        Game {
            id:   "slots".to_string(),
            name: "Slots".to_string(),
            game: games::slots::Slots::new(&conf.slots),
        },
        //Game {
        //    id:   "roulette".to_string(),
        //    name: "Roulette".to_string(),
        //    game: games::roulette::Roulette::new(&conf.roulette),
        //},
    ];
    let mut selection = Selection::new(&game_list);
    let mut status = Status::Selecting;

    println!("{}", game_list[selection.current]);

    for sig in signals.forever() {
        match status {
            Status::Selecting => status = selection.signal(sig),
            Status::Balancing => status = account.signal(sig),
            Status::Gaming(id) => {
                status = game_signal(&mut game_list, id, sig, &mut account)
            }
        }
        match status {
            Status::Selecting => println!("{}", game_list[selection.current]),
            Status::Balancing => {
                println!("Balance: {}, Bet: {}", account.balance, account.bet)
            }
            Status::Gaming(id) => println!("{}", game_list[id].game),
        }
    }

    println!("Come back soon!");
}

pub enum Status {
    Gaming(usize),
    Balancing,
    Selecting,
}

struct Selection {
    current: usize,
    total:   usize,
}

impl Selection {
    fn new<T>(list: &[Game<T>]) -> Self
    where T: Display + Controls {
        Self {
            current: 0,
            total:   list.len(),
        }
    }

    fn scroll(&mut self, side: bool) {
        let new_current = if side {
            self.current as isize - 1
        } else {
            self.current as isize + 1
        };
        self.current = new_current.wrapping_rem(self.total as isize) as usize;
    }

    fn signal(&mut self, sig: i32) -> Status {
        match sig {
            SIGCONT => return Status::Gaming(self.current),
            SIGINT  => return Status::Balancing,
            SIGUSR1 => self.scroll(false),
            SIGUSR2 => self.scroll(true),
            _       => {},
        }
        Status::Selecting
    }
}

fn game_signal<T>(game_list: &mut [Game<T>],
                  id: usize,
                  sig: i32,
                  acc: &mut Account) -> Status
where T: Controls + Display {
    match game_list[id].play(sig) {
        Loop::InGame(result) => if let Some(pl) = result {acc.e_bal(pl)},
        Loop::Balance => return Status::Balancing,
        Loop::Exit => return Status::Selecting,
    }
    Status::Gaming(id)
}

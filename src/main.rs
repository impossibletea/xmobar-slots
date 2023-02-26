use std::{process, fmt::Display};
use serde::{Serialize, Deserialize};
use signal_hook::{consts::signal::*, iterator::Signals};

mod games;
use games::{Game, Controls};

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
    let mut account = games::Account::new(&conf.account);
    let game_list = [
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
    let mut game: Option<usize> = None;

    for sig in signals.forever() {
        match game {
            Some(id) => {
                if let Some(pl) = game_list[id].play(sig) {account.e_bal(pl)}
            }
            None => {
                println!("{}", game_list[selection.current]);
                game = selection.signal(sig);
            }
        }
    }

    println!("Come back soon!");
}

struct Selection {
    current: usize,
    total:   usize,
}

impl Selection {
    fn new<T: Display + Controls>(list: &[Game<T>]) -> Self {
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

    fn signal(&mut self, sig: i32) -> Option<usize> {
        match sig {
            SIGCONT => return Some(self.current),
            SIGUSR1 => self.scroll(false),
            SIGUSR2 => self.scroll(true),
            _       => {},
        }
        None
    }
}


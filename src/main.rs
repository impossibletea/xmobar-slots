use signal_hook::{consts::signal::*, iterator::Signals};
use std::process;
use serde::{Serialize, Deserialize};

mod games;

#[derive(Serialize, Deserialize)]
struct Config {
    account: games::AccConfig,
    slots: games::slots::SlotConfig,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            account: games::AccConfig::default(),
            slots: games::slots::SlotConfig::default(),
        }
    }
}

fn main() {
    let conf = confy::load("xmobet", "config").unwrap_or(Config::default());

    let inputs = [
        SIGCONT, // Enter
        SIGUSR1, // Scroll up
        SIGUSR2, // Scroll down
        SIGINT, // Esc
    ];
    let mut signals = match Signals::new(&inputs) {
        Ok(result) => result,
        Err(err) => {
            println!("{err}");
            process::exit(1)
        }
    };

    for sig in signals.forever() {
    }

    println!("Come back soon!");
}


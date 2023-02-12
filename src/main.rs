use signal_hook::{consts::signal::*, iterator::Signals};
use std::process;
use serde::{Serialize, Deserialize};

mod slots;
mod game;

#[derive(Serialize, Deserialize)]
pub struct Config {
    drums: usize,
    symbols: String,
    init_balance: isize,
    init_bet: usize,
    bet_inc: usize,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            drums: 5,
            symbols: String::from("7JQKA"),
            init_balance: 100,
            init_bet: 5,
            bet_inc: 1,
        }
    }
}

fn main() {
    let conf = confy::load("xmobet", "config").unwrap_or(Config::default());

    // Catch signals
    let inputs = [
        SIGCONT, // Play
        SIGUSR1, // Increase bet
        SIGUSR2, // Decrease bet
        SIGINT, // Show balance
    ];
    let mut signals = match Signals::new(&inputs) {
        Ok(result) => result,
        // I guess this fails on non-linux???
        Err(err) => {
            println!("{err}");
            process::exit(1)
        }
    };

    // Set the initial state of slots
    let mut machine = slots::Slots::new(&conf);
    let mut player = game::Account::new(&conf);

    // Initial display
    game::home(&machine, &player.bet);

    // Game loop
    for sig in signals.forever() {
        match sig {
            // Round
            SIGCONT => game::round(&mut player, &mut machine),
            // Change stake
            SIGUSR1 => {
                player.e_bet(true);
                game::home(&machine, &player.bet)
            }
            SIGUSR2 => {
                player.e_bet(false);
                game::home(&machine, &player.bet)
            }
            // Show balance
            SIGINT => {
                println!("Balance: {}", player.balance);
                game::pause();
                game::home(&machine, &player.bet)
            }
            // Exit
            _ => break,
        }
    }

    // A kind goodbye
    println!("Kill yourself!");
}


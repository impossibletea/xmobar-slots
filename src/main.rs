use signal_hook::{consts::signal::*, iterator::Signals};
use std::process;

mod slots;
mod game;

pub struct Config {}

fn main() {
    let conf = Config{};

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
    let mut machine = slots::Slots::new(3);
    let mut player = game::Account::new(conf);

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


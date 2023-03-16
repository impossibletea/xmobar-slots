use std::process;
use serde::{Serialize, Deserialize};
use signal_hook::{consts::signal::*, iterator::Signals};

mod games;
use games::{Game, Account, Loop};

#[derive(Serialize, Deserialize)]
pub struct Config {
    welcome:  String,
    bye:      String,
    account:  games::AccConfig,
    slots:    games::slots::SlotConfig,
    roulette: games::roulette::RouletteConfig,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            welcome:  "Welcome to the Cum Zone!".to_string(),
            bye:      "Come back soon!".to_string(),
            account:  games::AccConfig::default(),
            slots:    games::slots::SlotConfig::default(),
            roulette: games::roulette::RouletteConfig::default(),
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
    let mut game_list: Vec<Game> = vec![
        Box::new(games::slots::Slots::new(&conf.slots)),
        Box::new(games::roulette::Roulette::new(&conf.roulette)),
    ];
    let mut selection = Selection::new(&game_list);
    let mut status = Status::Selecting;

    println!("{}", conf.welcome);
    games::pause();
    println!("{}", game_list[selection.current].name());

    for sig in signals.forever() {
        match status {
            Status::Selecting  => status = selection.signal(sig),
            Status::Balancing  => status = account.signal(sig),
            Status::Gaming(id) => status = game_signal(&mut game_list,
                                                       id,
                                                       sig,
                                                       &mut account)
        }
        match status {
            Status::Selecting => println!("{}",
                                          game_list[selection.current].name()),
            Status::Balancing => println!("Balance: {}, Bet: {}",
                                          account.balance, account.bet),
            Status::Gaming(id) => println!("{}", game_list[id])
        }
    }

    println!("{}", conf.bye);
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
    fn new(list: &Vec<Game>) -> Self {
        Self {
            current: 0,
            total:   list.len(),
        }
    }

    fn scroll(&mut self, side: bool) {
        let change: isize = if side {1} else {-1};
        self.current = match self.current.checked_add_signed(change) {
            Some(result) => result % self.total,
            None         => self.total - 1
        }
    }

    fn signal(&mut self, sig: i32) -> Status {
        match sig {
            SIGCONT => return Status::Gaming(self.current),
            SIGINT  => return Status::Balancing,
            SIGUSR1 => self.scroll(false),
            SIGUSR2 => self.scroll(true),
            _       => {}
        }
        Status::Selecting
    }
}

fn game_signal(game_list: &mut Vec<Game>,
               id: usize,
               sig: i32,
               acc: &mut Account) -> Status {
    match game_list[id].play(sig) {
        Loop::InGame(result) => if let Some(pl) = result {acc.e_bal(pl)},
        Loop::Balance        => return Status::Balancing,
        Loop::Exit           => return Status::Selecting
    }
    Status::Gaming(id)
}

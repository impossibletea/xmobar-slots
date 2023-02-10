use random::Source;
use signal_hook::{consts::signal::*, iterator::Signals};
use std::{
    iter::zip,
    process,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
    collections::HashMap,
};

// Could just return a number, but this makes it more obvious
enum PL {
    Profit(usize),
    Loss,
}

// The absolute state
struct Slots {
    status: Vec<usize>,
    ndrums: usize,
    drum: Vec<char>,
}

impl Slots {
    fn new(ndrums: usize) -> Self {
        Slots {
            status: vec![0; ndrums],
            // Next two will be customizable via config later
            ndrums,
            drum: "A7JQK".chars().collect(),
        }
    }
    fn roll(&mut self) -> () {
        // Set up random
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => 69, //nice
        };
        let mut source = random::default(seed);

        // Setting up random times between drum stops
        // 25 to 44 initial rolls
        let mut roll_time = || {source.read_u64() % 20 + 25};
        let mut roll_times: Vec<u64> = vec![roll_time(); self.ndrums];
        for i in 0..self.ndrums {
            let plus = source.read_u64() % 5 + 5; // 5 to 9 splits
            for element in roll_times[0..i].iter_mut() { *element += plus }
        }
        roll_times.reverse();

        // Get the largest roll count for iteration
        let longest = match roll_times.last() {
            Some(time) => time.clone(),
            None => 69,
        };

        // Iteratively print untl all stop
        for time in 0..longest {
            // Parallel iteration of drums and corresponding statuses
            for (wheel, limit) in zip(&mut *self.status, &roll_times) {
                // Status update either +1 or not if hits limit
                if time < *limit { *wheel = (*wheel + 1) % self.drum.len() }
            }
            // Time interval between status update
            println!("{self}");
            sleep(Duration::from_millis(50));
        }
    }
}

// Only care about showing slots
// Also to_string() for free for later use
impl std::fmt::Display for Slots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self.status
            .iter()
            .map(|n| {self.drum[*n]})
            .collect();
        write!(f, "{display}")
    }
}

fn main() {
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
    let mut machine = Slots::new(3);

    let mut balance = get_balance();
    let mut bet: usize = 5;

    // Initial display
    println!("{machine} Bet: {bet}");

    // Game loop
    for sig in signals.forever() {
        match sig {
            // Round
            SIGCONT => game_round(&mut balance, &bet, &mut machine),
            // Change stake
            SIGUSR1 => {bet += 5; println!("Bet: {bet}")}
            SIGUSR2 => {bet -= 5; println!("Bet: {bet}")}
            // Show balance
            SIGINT => println!("Balance: {balance}"),
            // Exit
            _ => break,
        }
        pause();
        println!("{machine} Bet: {bet}");
    }

    // A kind goodbye
    println!("Kill yourself!");
}

// This is overkill, but just in case I want to get balance in a different way
fn get_balance() -> usize {
    let balance = 1000;
    balance
}

fn game_round(balance: &mut usize, bet: &usize, machine: &mut Slots) -> () {
    machine.roll();
    pause();
    let result = combination_check(&machine.to_string());
    match result {
        // Most gamblers quit right before they're about to hit it big
        PL::Profit(multi) => {
            let win = bet * multi;
            *balance += win;
            println!("You win {win}!")
        }
        PL::Loss => {
            *balance -= bet;
            println!("Broke ass")
        }
    }
}

// Convenience
fn pause() {
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


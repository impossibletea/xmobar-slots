use random::Source;
use signal_hook::{consts::signal::*, iterator::Signals};
use std::{
    iter::zip,
    process,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

enum PL {
    Profit(usize),
    Loss,
}

struct Slots {
    status: Vec<u64>,
    ndrums: usize,
    drum: Vec<char>,
}

impl Slots {
    fn new(ndrums: usize) -> Self {
        Slots {
            status: vec![0; ndrums],
            ndrums,
            drum: "0123456789".chars().collect(),
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
        let mut roll_times: Vec<u64> = vec![
        source.read_u64() % 20 + 25; // 25 to 44 initial rolls
        self.ndrums
    ];
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
                if time < *limit { *wheel = *wheel + 1 % 10 }
            }
            // Time interval between status update
            sleep(Duration::from_millis(50));
        }
    }
}

fn main() {
    // Catch signals
    let mut signals = match Signals::new(&[SIGINT, SIGUSR1]) {
        Ok(result) => result,
        // I guess this fails on non-linux???
        Err(err) => {
            println!("{err}");
            process::exit(1)
        }
    };

    // Set the initial state of slots
    let mut machine = Slots::new(5);

    let mut balance = get_balance();
    let mut bet: usize = 5;

    // Game loop
    for sig in signals.forever() {
        match sig {
            // Round
            SIGUSR1 => game_round(&mut balance, &bet, &mut machine),
            // Change stake
            SIGUSR2 => todo!(),
            // Exit
            _ => break,
        }
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
}

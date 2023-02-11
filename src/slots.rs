use random::Source;
use std::{
    iter::zip,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// The absolute state
pub struct Slots {
    status: Vec<usize>,
    ndrums: usize,
    drum: Vec<char>,
}

impl Slots {
    pub fn new(ndrums: usize) -> Self {
        Slots {
            status: vec![0; ndrums],
            // Next two will be customizable via config later
            ndrums,
            drum: "A7JQK".chars().collect(),
        }
    }
    pub fn roll(&mut self) -> () {
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

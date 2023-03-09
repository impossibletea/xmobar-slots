use std::{
    iter::zip,
    thread::sleep,
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use random::Source;
use signal_hook::consts::signal::*;
use serde::{Serialize, Deserialize};
use crate::games::{self, PL, Controls, Loop, Playable};

#[derive(Serialize, Deserialize)]
pub struct SlotConfig {
    drums:   usize,
    symbols: String,
}

impl std::default::Default for SlotConfig {
    fn default() -> Self {
        Self {
            drums:   3,
            symbols: "7JQKA".to_string(),
        }
    }
}

pub struct Slots {
    status: Vec<usize>,
    ndrums: usize,
    drum:   Vec<char>,
}

impl Slots {
    pub fn new(conf: &SlotConfig) -> Self {
        Slots {
            status: vec![0; conf.drums],
            ndrums: conf.drums,
            drum:   conf.symbols.chars().collect(),
        }
    }

    fn roll(&mut self) -> () {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n)  => n.as_secs(),
            Err(_) => 69, //nice
        };
        let mut source = random::default(seed);

        let mut roll_time = || {source.read_u64() % 20 + 25};
        let mut roll_times: Vec<u64> = vec![roll_time(); self.ndrums];
        for i in 0..self.ndrums {
            let plus = source.read_u64() % 5 + 5;
            for element in roll_times[0..i].iter_mut() {*element += plus}
        }
        roll_times.reverse();

        let longest = match roll_times.last() {
            Some(time) => time.clone(),
            None       => 69
        };

        for time in 0..longest {
            for (wheel, limit) in zip(&mut *self.status, &roll_times) {
                if time < *limit {*wheel = (*wheel + 1) % self.drum.len()}
            }
            println!("{self}");
            sleep(Duration::from_millis(50));
        }
    }

    fn combination_check(&self) -> PL {
        let mut totals = HashMap::<char, usize>::new();

        for i in self.to_string().chars() {
            totals.entry(i).and_modify(|n| {*n += 1}).or_insert(0);
        }

        match totals.values().max() {
            Some(k) if k > &0 => PL::Profit(*k),
            _                 => PL::Loss
        }
    }
}

impl Playable for Slots {
    fn name(&self) -> String {"Slot machine".to_string()}
}

impl std::fmt::Display for Slots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self.status
            .iter()
            .map(|n| {self.drum[*n]})
            .collect();
        write!(f, "{display}")
    }
}

impl Controls for Slots {
    fn play(&mut self, sig: i32) -> Loop {
        match sig {
            SIGCONT => {
                self.roll();
                games::pause();
                Loop::InGame(Some(self.combination_check()))
            }
            SIGINT => Loop::Balance,
            _      => Loop::Exit
        }
    }
}


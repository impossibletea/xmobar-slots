use serde::{Serialize, Deserialize};
use crate::games::{self, PL, Controls, Loop, Playable};

#[derive(Serialize, Deserialize)]
pub struct RouletteConfig {
    double_zero: bool,
}

impl std::default::Default for RouletteConfig {
    fn default() -> Self {
        Self {
            double_zero: false,
        }
    }
}

pub struct Roulette {
}

impl Roulette {
    pub fn new(conf: &RouletteConfig) -> Self {
        Self {
        }
    }
}

impl Playable for Roulette {
    fn name(&self) -> String {"Roulette".to_string()}
}

impl std::fmt::Display for Roulette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Controls for Roulette {
    fn play(&mut self, sig: i32) -> Loop {
        todo!()
    }
}


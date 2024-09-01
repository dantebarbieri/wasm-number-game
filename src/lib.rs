mod utils;

use rand::rngs::OsRng;
use rand::Rng;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub enabled: bool,
    value: Option<u32>,
}

#[wasm_bindgen]
pub struct Game {
    slots: Vec<Slot>,
    pool: Vec<u32>,
    min: u32,
    max: u32,
}

fn create_pool(count: usize, lo: u32, hi: u32) -> Vec<u32> {
    let mut rng = OsRng;
    let mut unique_numbers = HashSet::new();

    while unique_numbers.len() < count {
        let num = rng.gen_range(lo..=hi);
        unique_numbers.insert(num);
    }

    unique_numbers.into_iter().collect()
}

use std::fmt;

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(x) = self.value {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Slot {
    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new(count: usize, lo: u32, hi: u32) -> Game {
        utils::set_panic_hook();

        let slots = std::iter::repeat(Slot {
            enabled: true,
            value: None,
        })
        .take(count)
        .collect();

        Game {
            slots,
            pool: create_pool(count, lo, hi),
            min: lo,
            max: hi,
        }
    }

    pub fn step(&mut self, idx: usize) {
        self.place(idx).unwrap();

        self.update_enabled();
    }

    pub fn hint(&self) -> Option<usize> {
        let slots = self.num_available();
        if slots == 0 {
            return None;
        }

        match self.limit_vals() {
            Some((a, b)) => {
                let range = (1 + b - a) as f64;
                let bucket_size = range / slots as f64;
                let next = self.next().unwrap() as f64;

                let i = (0..slots)
                    .take_while(|&i| next > (a as f64 + ((i + 1) as f64 * bucket_size)))
                    .count();

                let s = self.limits().unwrap().0;

                Some(s + i)
            }
            None => None,
        }
    }

    pub fn tick(&mut self) {
        if let Some(idx) = self.hint() {
            self.step(idx);
        }
    }

    pub fn slot(&self, idx: usize) -> Slot {
        self.slots[idx]
    }

    pub fn next(&self) -> Option<u32> {
        self.pool.last().copied()
    }

    pub fn num_filled(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| slot.value.is_some())
            .count()
    }

    pub fn num_available(&self) -> usize {
        self.slots.iter().filter(|slot| slot.enabled).count()
    }
}

impl Game {
    fn place(&mut self, idx: usize) -> Result<(), String> {
        match self.slots[idx] {
            Slot {
                enabled: false,
                value: _,
            } => Err("slot is not enabled".into()),
            Slot {
                enabled: true,
                value: Some(_),
            } => Err("slot contains a value".into()),
            Slot {
                enabled: true,
                value: None,
            } => {
                self.slots[idx].value = self.pool.pop();
                Ok(())
            }
        }
    }

    fn limits(&self) -> Option<(usize, usize)> {
        match self.pool.last().copied() {
            Some(next_output) => {
                let mut s: Option<usize> = None;
                let mut t: Option<usize> = None;

                for (i, slot) in self.slots.iter().enumerate() {
                    if let Some(slot_value) = slot.value {
                        // Update `s` if the current slot's value is less than `next_output`
                        if slot_value < next_output {
                            s = Some(i + 1);
                        }

                        // Set `t` if `t` is None and the current slot's value is greater than `next_output`
                        if t.is_none() && slot_value > next_output {
                            t = Some(i);
                        }
                    }
                }

                let s = s.unwrap_or(0);
                let t = t.unwrap_or(self.slots.len());
                Some((s, t))
            }
            None => None,
        }
    }

    fn limit_vals(&self) -> Option<(u32, u32)> {
        match self.limits() {
            Some((s, t)) => {
                let a = if s == 0 {
                    self.min
                } else {
                    self.slots[s - 1].value.unwrap_or(self.min)
                };
                let b = if t == self.slots.len() {
                    self.max
                } else {
                    self.slots[t].value.unwrap_or(self.max)
                };
                Some((a, b))
            }
            None => None,
        }
    }

    fn update_enabled(&mut self) {
        if let Some((s, t)) = self.limits() {
            for (i, slot) in self.slots.iter_mut().enumerate() {
                slot.enabled = s <= t && s <= i && i < t;
            }
        } else {
            for slot in self.slots.iter_mut() {
                slot.enabled = false;
            }
        }
    }
}

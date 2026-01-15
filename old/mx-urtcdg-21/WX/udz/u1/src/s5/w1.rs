/*
Section 5 - Work 1.rs
- First file which will have the major work
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function---

pub fn s5_w1_main() {
    greet();
}

// --- Sub Functions --

// greeter function
fn greet() {
    pswg("Sec5 - Media Cataglo Enum Patterns".to_string())
}

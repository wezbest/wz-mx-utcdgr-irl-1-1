/*
Main entry point for the Rust application.
*/

mod s7;
mod utils;

// use s2::s2::s2_main as s2m;
// use s5::w1::s5_w1_main as s5m;
// use s5::l1::s5_l1_main as s5lm;
use s7::w1::s7_w1_main as s7mw1;
// use s7::l1::s7_l1_main as s7ml1;

fn main() {
    s7mw1();
}

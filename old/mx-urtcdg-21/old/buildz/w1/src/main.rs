// -------------------------------------------------
// Main Entry point
// -------------------------------------------------

//--- Attributes ---
#![allow(dead_code)]
#![allow(unused_imports)]

// ---Imports ---
mod sec2;
mod tez;
mod utilz;

use sec2::s1::{self, s1_main};
use tez::box1::main_tez;

// -- Exec----
fn main() {
    s1_main();
}

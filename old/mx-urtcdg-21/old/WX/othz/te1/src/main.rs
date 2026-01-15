// ------------------------------------
// Main Entry point of the Application
// -----------------------------------

// Imports

use utilz::{clear_console, header, pswg};
mod utilz;

// Main Logic

fn main() {
    clear_console();
    pswg("Whootey".to_string());
    header("Wilka");
    println!("Hello, world!");
}

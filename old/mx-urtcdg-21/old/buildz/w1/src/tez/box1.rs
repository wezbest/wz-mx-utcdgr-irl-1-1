// ----------------------------------------------
// tez/box1.rs - Testing out boxy cli
// ----------------------------------------------

// -- Attributes ---
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, header, pswg};
use boxy_cli::prelude::*;

// -- Main function ---

pub fn main_tez() {
    clear_console();
    boxy2()
}

// -- Subs

// Testing out boxy-
fn boxy1() {
    let t1 = "Boxy Test 1";
    pswg(t1.to_string());

    let mut my_box = Boxy::builder()
        .box_type(BoxType::Double) // Set border style
        .color("#00ffff") // Set border color
        .padding(
            BoxPad::uniform(1),            // External padding
            BoxPad::from_tldr(2, 2, 1, 1), // Internal padding
        )
        .align(BoxAlign::Left) // Center the box in the terminal
        .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
        .add_line("This is a new line.", "#32CD32")
        .add_segment("Another section", "#663399", BoxAlign::Left)
        .width(40) // Set fixed width
        .build();
    my_box.display();
}

// Box test with information
fn boxy2() {
    let t2 = "Boxy Test 2 - with data";
    pswg(t2.to_string());

    let vec1 = vec!["Item 1", "Item 2", "Item 3"];

    let mut my_box = Boxy::builder()
        .box_type(BoxType::Double) // Set border style
        .color("#00ffff") // Set border color
        .padding(
            BoxPad::uniform(1),            // External padding
            BoxPad::from_tldr(2, 2, 1, 1), // Internal padding
        )
        .align(BoxAlign::Left) // Center the box in the terminal
        .add_segment("Variables Test", "#ffffff", BoxAlign::Center)
        .add_segment("Printing vector", "#663399", BoxAlign::Left)
        .add_line(&vec1.join(", "), "#32CD32")
        .width(40) // Set fixed width
        .build();
    my_box.display();
}

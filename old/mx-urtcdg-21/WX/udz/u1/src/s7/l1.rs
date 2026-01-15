/*
l1.rs -
1. All learning related code here
*/

// -- Directives ---
#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use std::io::Error; // Required for custom error handling
use yansi::Paint;

// --- Main Function ---

pub fn s7_l1_main() {
    greet();
    // div1();
    // div2();
    validate_email_function("jana@hah.com".to_string());
}

// --- Sub Functions ---

fn greet() {
    pswg("s7/l1.rs - Learning related code here".to_string())
}

// --- Sub Functions ---

// Divid function for learning about enums

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Fucker no division by zero"))
    } else {
        Ok(a / b)
    }
}

// Implement above function

fn div1() {
    header("Div1 - Function");

    let val1 = 10.0;
    let val2 = 3.0;

    let a1 = divide(val1, val2);

    // note you need unwrap() to get the value out of the result
    println!("{} = {}", "10/2 is ", a1.unwrap().blue());
}

//div2

fn div2() {
    header("Div 2 ");

    match divide(12.0, 100.0) {
        Ok(result_of_division) => {
            println!("{}", "12/3 is ".blue());
            println!("{}", result_of_division.to_string().blue());
        }
        Err(what_went_wrong) => {
            println!("{}", "Error: ".red());
            println!("{}", what_went_wrong.to_string().red());
        }
    }
}

/// Validate enums  - simple validation check

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(Error::other("Email must have @ bastard"))
    }
}

// Function using above with match
fn validate_email_function(email: String) {
    header("Validate Email");
    // let email = String::from("hello@world.com");
    match validate_email(email) {
        Ok(..) => {
            println!("{}", "Email is valid".green());
        }
        Err(e) => {
            println!("{}", "Email is invalid".red());
            println!("{}", e.to_string().red());
        }
    }
}

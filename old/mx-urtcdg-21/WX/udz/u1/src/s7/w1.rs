/*
Section 7 - Section 7 work here
- Focus is on making various logs
- Focus on handling errors
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use core::error;
use rand::{rng, seq::SliceRandom};
use std::fs;
use std::io::Error;
use yansi::Paint;

// --- Main Function ---

pub fn s7_w1_main() {
    greet();
    // func1();
    // func2();
    // func3();
    // func4();
    // func4_extract_error();
    // func5();
    // func6();
    func7();
}

// --- Sub Functions ---

fn greet() {
    pswg("Sec7 - Section 7 work here".to_string())
}

// Open and reading the file

fn func1() {
    header("Opening and Reading the file");
    let file = fs::read_to_string("src/s7/s7logs.txt").unwrap();

    // note this formatter is for debugging of data strcuts
    println!("{}", "---Printing With formatter---".on_blue());
    println!("{:#?}", file.yellow());

    // Normally print out to terminal done like this
    println!("{}", "---Printing w/o formatter ---".on_blue());
    println!("{}", file.blue());
}

// Same as above functions using match statement
fn func2() {
    header("Using match statement");

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            println!("{}", "---Printing With formatter Characters---".on_blue());
            println!("{:#?}", file.len().yellow());
            println!("{}", "---Printing Full File---".on_blue());
            println!("{:#?}", file.yellow());
        }
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }
}

// test function for strings

// fn string_test(a: String, b: &String, c: &str) {}
fn func3() {
    header("Using match statement");

    let mut error_logs = vec![];

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            error_logs = func4_extract_error(file.as_str());
        } // file will be dropped here that why error
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }

    println!("{:#?}", error_logs);
    println!("No of lines: {:#?}", error_logs.len().yellow());
}

/*
For extracting error part , will again copy the function
- Then the extract errors will be used on it
*/

fn func4() {
    header("Extracting Errors");

    // string_test(String::from("Panty"), &String::from("Panty"), "Panty");

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            println!("{}", "---Printing With formatter Characters---".on_blue());
            println!("{:#?}", file.len().yellow());
            println!("{}", "---Printing Full File---".on_blue());
            println!("{:#?}", file.yellow());
        }
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }
}

// Extrction function

fn func4_extract_error(text: &str) -> Vec<String> {
    // header("Text Extraction");

    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.to_lowercase().starts_with("error") {
            results.push(line.to_string());
        }
    }

    results
}

// Conituation from Lesson 70

fn func5() {
    header("Using match statement - Fucntion 5");

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            let error_logs = func4_extract_error(file.as_str());
            let error_logs_content = error_logs.join("\n");

            // Display contents before writing
            println!("{}", "[=] Contents to be written:".blue());
            println!("{}", error_logs_content);

            // Writing to  file
            match fs::write("src/s7/s7errorlogs.txt", error_logs.join("\n")) {
                Ok(..) => println!("{}", "[+] File written successfully".green()),
                Err(e) => {
                    eprintln!("{}", "Error: ".red());
                    eprintln!("{}", e.to_string().red());
                }
            }
        }
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }
}

// Altarntive to Match Statemnts

fn func6() {
    header("Using match statement - Fucntion 6");

    let text = fs::read_to_string("src/s7/s7logs.txt").expect("[!] Fucked");

    let error_logs = func4_extract_error(text.as_str());

    // Write directly to file
    fs::write("src/s7/s7errorlogs.txt", error_logs.join("\n")).expect("[!] Fucked Not Written");
}

// Yest another alternative using unwrap_or_else
fn func7() -> Result<(), Error> {
    header("Using match statement - Fucntion 6");

    let text = fs::read_to_string("src/s7/s7logs.txt")?;
    println!("{}", text.len());
    let error_logs = func4_extract_error(text.as_str());
    fs::write("src/s7/s7errorlogs.txt", error_logs.join("\n"))?;

    Ok(())

    // // Write directly to file

    // Err(Error::other("smelling..panty.."))
}

#!/usr/bin/env bun

/* 
Section3 Chapter 20 Work
*/

// --- Imports ---
import boxen from "boxen"
import chalk from "chalk"

// --- Main Function ---

function title(textz, titlez) {
  console.log(
    boxen(chalk.yellow(textz), {
      padding: 0.7,
      textAlign: "center",
      title: titlez,
      titleAlignment: "center",
      borderColor: "green",
      borderStyle: "doubleSingle",
    })
  )
}

function s3main() {
  title("Section 3 - Chapter 20", "JS Booty Smell Test")
  console.log(chalk.blue("Testing JS Booty Smell..."))
}
s3main()

// -- Sub Function called in Main ---

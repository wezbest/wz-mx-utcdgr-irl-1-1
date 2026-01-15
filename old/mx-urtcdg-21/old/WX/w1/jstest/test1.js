#!/usr/bin/env bun

/* 
This is for testing shell js scripts
*/

// --- Imports ---
import boxen from "boxen"
import chalk from "chalk"

// --- Variables ---

// --- Main Function Call ---

function main() {
  boxen1("smellybooty")
}

// -- Sub Function Calls

// Box Function - styler
function boxen1(textz, titlez) {
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

// Printing text function
function writeText(params) {
  console.log(chalk.blue("Hello world!"))
}

// --- execution zone ---
main()

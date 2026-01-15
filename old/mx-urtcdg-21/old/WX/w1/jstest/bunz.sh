#!/usr/bin/bash
# This bash srcript is for installing the KL docker image here
clear

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[0;37m'
export NC='\033[0m' # No Color

# Commands

hea1() {
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
}

# make a new project
cargo_new() {
    hea1 "Cargo New Initialization with yansi and cfonts"
    # Get name of Project
    echo -e "Enter the name of the project: "
    read name_of_project
    if [ -z "$name_of_project" ]; then
        echo -e "${RED}BASTARD ! Project name cannot be empty${NC}"
        exit 1
    fi

    # Commands to execute
    CO1="cargo new $name_of_project"
    CO2="cd $name_of_project"
    CO3="cargo add yansi cfonts"
    CO4="cargo tree"

    # Show Commands
    echo -e ""
    echo -e "${GREEN}---Commands to execute:---"
    echo -e ">$CO1"
    echo -e ">$CO2"
    echo -e ">$CO3"
    echo -e ">$CO4"
    echo -e "Executing....${NC}"

    # Execution Commands
    eval "$CO1"
    eval "$CO2"
    eval "$CO3"
    eval "$CO4"

}

# Remove all targets
cargo_remove_target() {
    hea1 "Cargo Clean Recursive"
    co1="find . -name "Cargo.toml" -execdir cargo clean \;"
    echo -e "${GREEN}Executing: $co1${NC}"
    eval "$co1"
}


# Make bunjs executable
# https://bun.com/docs/bundler/executables - Single File
bun_ex() {
    TARGET="test1.js"
    hea1 "Make Bun Executable"
    co1="bun build --compile --minify --sourcemap --bytecode ${TARGET} --outfile ${TARGET}_bun"
    echo -e "${GREEN}Executing: $co1${NC}"
    eval "$co1"
}

# Execution
# cargo_new
# cargo_remove_target
bun_ex

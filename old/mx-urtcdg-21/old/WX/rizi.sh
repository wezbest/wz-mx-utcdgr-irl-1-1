#!/usr/bin/bash
# Installing a lightweight CLI reverse engineering tool 
# 
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

# Rizin Install 

rizin_install() {
    hea1 "Rizin Reverse Engineering Toolkit"
    echo 'deb http://download.opensuse.org/repositories/home:/RizinOrg/Debian_Unstable/ /' | sudo tee /etc/apt/sources.list.d/home:RizinOrg.list
    curl -fsSL https://download.opensuse.org/repositories/home:RizinOrg/Debian_Unstable/Release.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/home_RizinOrg.gpg > /dev/null
    sudo apt update
    sudo apt install rizin
    rizin --help
}

# Execution
# cargo_new
# cargo_remove_target
rizin_install
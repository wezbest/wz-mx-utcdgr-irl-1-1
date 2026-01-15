#!/usr/bin/bash
# Runnign Actual Work Commands
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

# -- Var ---
BINARY="pantismyl"

# --- Coms ---
# Initial Information 
step1() {
    hea1 "Initial Investigation"

    c0="mkdir -p anal"
    c1="file ${BINARY} | tee -a anal/file1.txt"
    c2="readelf -h ${BINARY} | tee -a anal/readelf2.txt"
    c3="strings ${BINARY} | tee -a anal/strings3.txt"
    c4="objdump -d ${BINARY} | tee -a anal/objdump4.txt"
    c5="nm -c ${BINARY} | tee -a anal/nm5.txt"

    echo -e "${GREEN}---Commands to execute:---"
    echo -e ">$c0"
    eval "$c0"
    echo -e ">$c1"
    eval "$c1"
    echo -e ">$c2"
    eval "$c2"
    echo -e ">$c3"
    eval "$c3"
    echo -e ">$c4"
    eval "$c4"
    echo -e ">$c5"
    eval "$c5"
    echo -e "Execution completed. Check the output files for details.${NC}"
}

# --- Execution ---
step1
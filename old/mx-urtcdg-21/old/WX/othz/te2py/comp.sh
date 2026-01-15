#!/usr/bin/bash
# Using various options for compression
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

# SubComs

# Using Quagga , damn fast
qua1() {
    hea1 "Quagga - Compression Test"
    DIRZ="car/reports_md/"
    co1="quagga --max-total-size=10000000 --max-part-size=10000000 ${DIRZ} -o q.txt"
    echo -e "${YELLOW}Executing: ${NC}${co1}"
    eval "$co1"
}

# Using repomix


# Execute 
qua1
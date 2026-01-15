#!/usr/bin/bash
# Prep for release 
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

# Release prep 
prep1() {
    clear
    hea1 "Prepare for release"
    TARGET="../WX/w1"
    echo -e "${YELLOW}Cleaning up build directories...${NC}"
    co1="rm -rf ${TARGET}/target/"
    eval "$co1"
    echo -e "${YELLOW}Copying TARGET to buildz...${NC}"
    co2="cp -r ${TARGET} ."
    eval "$co2"
    co3="git add . && git commit -m "👙" && git push"
    eval "$co3"
}

# Getting the built from GH 
get_filez() {
    hea1 "Getting the files from release"
    VER="v4.0.4"
    DEST="https://github.com/m0x0m0x/mx-urtcdg-2/releases/download/${VER}/pantismyl"
    co1="wget ${DEST}"
    echo -e "${GREEN} Executing: ${co1} " 
    eval "$co1"
}

# Execute 
# prep1
get_filez
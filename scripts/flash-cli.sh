#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
"" | "release")
    #bash scripts/build.sh
    BUILD_MODE="release"
    ;;
"debug")
    #bash scripts/build.sh debug
    BUILD_MODE="debug"
    ;;
*)cd 
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

cargo espflash flash --chip esp32c6 --list-all-ports --monitor --before default-reset --after hard-reset  #--bin target/riscv32imac-unknown-none-elf/${BUILD_MODE}/blinky_no_std 

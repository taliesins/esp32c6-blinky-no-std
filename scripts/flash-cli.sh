#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
"" | "release")
    #bash scripts/build.sh
    #cargo espflash partition-table --to-binary --output partitions/partitions.bin partitions.csv 
    BUILD_MODE="release"
    ;;
"debug")
    #bash scripts/build.sh debug
    #cargo espflash partition-table --to-binary --output partitions/partitions.bin partitions.csv 
    BUILD_MODE="debug"
    ;;
*)cd 
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

cargo espflash flash --monitor --before default-reset --after hard-reset --bootloader partitions/bootloader.bin --bin blinky_no_std # --partition-table partitions.csv # --partition-table-offset 0x10000 --target-app-partition ota_0 


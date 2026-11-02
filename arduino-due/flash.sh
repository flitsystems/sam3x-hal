#!/usr/bin/env bash
# Cargo runner for the Arduino Due's programming port:
#   1. convert the ELF to a raw binary;
#   2. open the port at 1200 baud: the board's 16U2 erases the SAM3X and
#      starts its SAM-BA bootloader;
#   3. wait until SAM-BA answers, then write and verify with bossac;
#   4. reset the SAM3X by writing RSTC_CR through SAM-BA, until SAM-BA no
#      longer answers. bossac's -R does not reset the board reliably.
#
#   DUE_PORT=/dev/ttyACM0 cargo run --release --bin blinky
set -euo pipefail

elf="$1"
port="${DUE_PORT:-/dev/ttyACM0}"
bin="${elf}.bin"

objcopy=""
for candidate in rust-objcopy arm-none-eabi-objcopy llvm-objcopy; do
    if command -v "$candidate" >/dev/null 2>&1; then
        objcopy="$candidate"
        break
    fi
done
if [[ -z "$objcopy" ]]; then
    echo "flash.sh: no objcopy found (cargo install cargo-binutils, or the Arm GNU toolchain)" >&2
    exit 1
fi
"$objcopy" -O binary "$elf" "$bin"

if [[ "$(uname)" == "Darwin" ]]; then
    stty_port=(stty -f "$port")
else
    stty_port=(stty -F "$port")
fi

# Whether SAM-BA answers on the port.
samba() {
    bossac --port="$port" -i >/dev/null 2>&1
}

# Open the port at 1200 baud and close it. hupcl drops DTR on the close,
# which the 16U2 needs to see. The 16U2 then erases the SAM3X and resets
# it into SAM-BA: leave it time to finish before opening the port again.
touch_1200() {
    "${stty_port[@]}" 1200 hupcl || true
    sleep 1.5
}

# Wait up to 2.5 s for SAM-BA.
wait_samba() {
    local tries=0
    until samba; do
        tries=$((tries + 1))
        if [[ $tries -ge 5 ]]; then
            return 1
        fi
        sleep 0.5
    done
}

# The touch or the first write after it sometimes fails: start again.
attempt=1
until touch_1200 && wait_samba && bossac --port="$port" -e -w -v -b "$bin"; do
    if [[ $attempt -ge 3 ]]; then
        echo "flash.sh: no SAM-BA on $port after $attempt attempts" >&2
        exit 1
    fi
    attempt=$((attempt + 1))
done

# RSTC_CR = KEY | PERRST | PROCRST.
tries=0
while samba; do
    tries=$((tries + 1))
    if [[ $tries -gt 3 ]]; then
        echo "flash.sh: the board stays in SAM-BA after the reset" >&2
        exit 1
    fi
    "${stty_port[@]}" 115200 raw -echo || true
    printf 'W400E1A00,A5000005#' >"$port" || true
    sleep 0.5
done

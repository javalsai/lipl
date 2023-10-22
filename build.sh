#!/usr/bin/env bash
set -e;
cd "$(dirname "$0")"

# ["key"]="nix-config rust-target"
declare -A options=(
    # ["x86_64"]="x86_64-unknown-linux-gnu x86_64-unknown-linux-gnu" #! Broken, lol (at least for me)
    ["aarch64"]="aarch64-unknown-linux-gnu aarch64-unknown-linux-gnu" # I think it's equivalent to ARMv8
    ["riscv64"]="riscv64-unknown-linux-gnu riscv64gc-unknown-linux-gnu"
    # ["riscv32"]="riscv32-unknown-linux-gnu riscv32gc-unknown-linux-gnu" #! Also Broken
    ["i686"]="i686-unknown-linux-gnu i686-unknown-linux-gnu" # AKA 32 bits iirc

    ["armv7a"]="armv7a-unknown-linux-gnueabihf armv7-unknown-linux-gnueabi"
    ["armv7a-hf"]="armv7a-unknown-linux-gnueabihf armv7-unknown-linux-gnueabihf" # RPi 2B

    ["other"]="$2 $3"
)

NIX_CONF="config.nix"
TEMPL_PROG="\e[0;33m%s \e[1;35m%s\e[0m\n"
TEMPL_DONE="\e[1;32m%s \e[1;35m%s\e[0m\n\n"
main() {
    trap "run git restore --staged '$NIX_CONF'" EXIT

    IFS=' ' read -r nix rust <<< "${options["$1"]}";
    # printf "Building for targets:\n nix: '%s'\n rust: '%s'\n\n" $nix $rust
    printf "$TEMPL_PROG" "Building for targets:" ""
    printf "$TEMPL_PROG" " nix:" "$nix"
    printf "$TEMPL_PROG" " rust:" "$rust"
    printf "\n"

    CONFIG_PATCH=$(printf '{ targets = { rust = "%s"; nix = "%s"; }; }\n' "$rust" "$nix")
    printf "$TEMPL_PROG" "Patching" "'$NIX_CONF'"
    pretty-print "$CONFIG_PATCH" nix
    printf "$CONFIG_PATCH" > "$NIX_CONF"
    printf "$TEMPL_DONE" "✓ Patched"

    run git add -f "$NIX_CONF"
    run nix build --extra-experimental-features flakes --extra-experimental-features nix-command
    run git restore --staged "$NIX_CONF"

    printf "\n"
    run mkdir -p out
    run cp result/bin/* out
    run chmod 775 out/*
    printf "\n"
    run patchelf --remove-rpath --set-interpreter /lib/ld-linux.so.3 out/* || \
    printf "\e[1;31mError patching elf, you'll have to do it yourself\e[0m\n"
    printf "$TEMPL_DONE" "✓ Done, binary at" out/*

    if command -v file &> /dev/null; then run file out/*; fi
    run ls -ldh out/*
}

pretty-print() {
    if command -v bat &> /dev/null; then
        bat -pl "$2" <<< "$1";
    else
        cat <<< "$1"
    fi
}

run() {
    printf "\e[30m\$ %s\e[0m\n" "$*"
    $@
}

for program in nix git; do
    command -v "$program" &> /dev/null || {
        printf "\e[1;31m%s\e[0m\n" "Command '$program' not found. ABORTING" >&2
        exit 1
    };
done

main $@

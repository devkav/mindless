#!/bin/bash

SCRIPT_PATH="${BASH_SOURCE[0]}"
DIR=$(dirname "$(realpath "$SCRIPT_PATH")")
BIN_PATH="$DIR/target/debug/mindless"

alias mindless=$BIN_PATH

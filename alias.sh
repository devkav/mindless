#!/bin/bash

SCRIPT_PATH="${BASH_SOURCE[0]:-${(%):-%x}}"
DIR=$(cd "$(dirname "$SCRIPT_PATH")" && pwd)
BIN_PATH="$DIR/target/debug/mindless"

alias mindless="$BIN_PATH"

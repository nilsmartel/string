#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "substr — chars 2..4" \
    "$(printf 'abcdef' | "$STRING" substr 2 4)" \
    "cd"

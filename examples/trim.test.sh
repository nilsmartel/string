#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "trim — strip whitespace and empty lines" \
    "$(printf '  hi  \n\n\tyo\t' | "$STRING" trim)" \
    "$(printf 'hi\nyo')"

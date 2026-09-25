#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# each replaces {} with every input line and runs the command
check "each — map lines through a command" \
    "$(printf '1\n2\n3' | "$STRING" each -- echo 'n{}')" \
    "$(printf 'n1\nn2\nn3')"

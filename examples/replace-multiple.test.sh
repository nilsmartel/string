#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "replace — multiple pairs" \
    "$(printf 'a b c' | "$STRING" replace a 1 c 3)" \
    "1 b 3"

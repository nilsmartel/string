#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# line indexes from 0, so index 1 is the second line
check "line — pick by index" \
    "$(printf 'zero\none\ntwo' | "$STRING" line 1)" \
    "one"

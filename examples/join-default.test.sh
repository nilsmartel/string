#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "join — default separator (space)" \
    "$(printf 'a\nb\nc' | "$STRING" join)" \
    "a b c"

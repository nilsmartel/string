#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "split — default separator (space)" \
    "$(printf 'a b c' | "$STRING" split)" \
    "$(printf 'a\nb\nc')"

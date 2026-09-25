#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "chars — one character per line" \
    "$(printf 'abc' | "$STRING" chars)" \
    "$(printf 'a\nb\nc')"

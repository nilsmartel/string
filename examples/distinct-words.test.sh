#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "distinct — words in order" \
    "$(printf 'a a b b c' | "$STRING" distinct)" \
    "$(printf 'a\nb\nc')"

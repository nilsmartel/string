#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "contains — keep matching lines" \
    "$(printf 'apple\nbanana\ncherry' | "$STRING" contains an)" \
    "banana"

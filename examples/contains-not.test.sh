#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "contains -n — keep non-matching lines" \
    "$(printf 'apple\nbanana\ncherry' | "$STRING" contains -n an)" \
    "$(printf 'apple\ncherry')"

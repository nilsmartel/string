#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "replace — single pair" \
    "$(printf 'foo bar' | "$STRING" replace bar baz)" \
    "foo baz"

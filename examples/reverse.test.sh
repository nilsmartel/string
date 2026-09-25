#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "reverse — line order" \
    "$(printf 'a\nb\nc' | "$STRING" reverse)" \
    "$(printf 'c\nb\na')"

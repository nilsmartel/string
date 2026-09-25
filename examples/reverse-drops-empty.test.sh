#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "reverse — drops empty lines" \
    "$(printf 'a\n\nb' | "$STRING" reverse)" \
    "$(printf 'b\na')"

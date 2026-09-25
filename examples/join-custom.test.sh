#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "join — custom separator (dash)" \
    "$(printf 'a\nb\nc' | "$STRING" join -)" \
    "a-b-c"

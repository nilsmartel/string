#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "case lower — mixed case" \
    "$(printf 'HeLLo' | "$STRING" case lower)" \
    "hello"

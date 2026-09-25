#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "starts-with — prefix filter" \
    "$(printf 'hello\nhelp\nworld' | "$STRING" starts-with hel)" \
    "$(printf 'hello\nhelp')"

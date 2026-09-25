#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "case upper — ascii" \
    "$(printf 'hello world' | "$STRING" case upper)" \
    "HELLO WORLD"

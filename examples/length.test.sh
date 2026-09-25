#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "length — ascii byte count" \
    "$(printf 'hello' | "$STRING" length)" \
    "5"

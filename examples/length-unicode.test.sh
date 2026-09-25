#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# length is measured in bytes: ä is two bytes in UTF-8
check "length — unicode byte count" \
    "$(printf 'ä' | "$STRING" length)" \
    "2"

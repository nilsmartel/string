#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "interleave — every 2nd line" \
    "$(printf '0\n1\n2\n3\n4' | "$STRING" interleave 2)" \
    "$(printf '0\n2\n4')"

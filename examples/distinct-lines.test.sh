#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "distinct -l — whole lines in order" \
    "$(printf 'x x\nx x\ny' | "$STRING" distinct -l)" \
    "$(printf 'x x\ny')"

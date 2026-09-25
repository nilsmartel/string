#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# substr counts characters, not bytes
check "substr — unicode chars 2..4" \
    "$(printf 'öüäß€' | "$STRING" substr 2 4)" \
    "äß"

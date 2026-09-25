#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "ends-with — suffix filter" \
    "$(printf 'cat\nbat\ndog' | "$STRING" ends-with at)" \
    "$(printf 'cat\nbat')"

#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# --sequential keeps input order even though the work runs on several threads.
# The inner `each` draws a progress bar on stderr (threads > 1); we only assert
# on stdout, so drop that noise to keep the test output clean.
check "each --threads --sequential — preserves input order" \
    "$(printf 'a\nb\nc' | "$STRING" each --threads=3 --sequential -- echo '{}' 2>/dev/null)" \
    "$(printf 'a\nb\nc')"

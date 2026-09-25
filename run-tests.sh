#!/bin/sh
# Run the example tests against the built `string` binary.
#
# Every examples/*.test.sh makes one assertion and exits 0 (pass) or 1 (fail).
set -u

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
STRING="$ROOT/target/debug/string"

[ -x "$STRING" ] || ( cd "$ROOT" && cargo build ) || exit 1
export STRING_BIN="$STRING"

find "$ROOT/examples" -name '*.test.sh' | "$STRING" each -t 8 --sequential -- sh '{}' 2> /dev/null

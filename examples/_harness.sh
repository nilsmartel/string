#!/bin/sh
# Shared test harness — sourced by every examples/*.test.sh
#
# Each test is a small script that makes a single assertion with `check` and
# exits 0 (pass) or 1 (fail). Keeping the logic here is what lets every test
# file look identical: source this, then call `check` once.

# Repo root, resolved from the test script's own path ($0 is the test file,
# even while this file is being sourced).
_here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(dirname -- "$_here")

# The binary under test. run-tests.sh exports STRING_BIN; fall back to the
# debug build so a test file can also be run on its own.
STRING="${STRING_BIN:-$ROOT/target/debug/string}"

# Colors, but only for a real terminal and when NO_COLOR is unset.
if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
    _green=$(printf '\033[32m')
    _red=$(printf '\033[31m')
    _dim=$(printf '\033[2m')
    _reset=$(printf '\033[0m')
else
    _green='' _red='' _dim='' _reset=''
fi

# Render a possibly multi-line string on one line, newlines shown as \n.
_oneline() {
    printf '%s' "$1" | awk 'BEGIN { ORS = "" } { if (NR > 1) printf "\\n"; print }'
}

# check <name> <actual> <expected>
#
# Passes are announced on stdout; failures (with a diff) go to stderr. That
# split matters for parallel runs: `string each` throws away a failing
# command's stdout and shows its stderr, so the diagnostic has to be on stderr
# to survive.
check() {
    _name=$1
    _actual=$2
    _expected=$3

    if [ "$_actual" = "$_expected" ]; then
        printf '%s✓%s %s\n' "$_green" "$_reset" "$_name"
        return 0
    fi

    {
        printf '%s✗%s %s\n' "$_red" "$_reset" "$_name"
        printf '    %sexpected:%s %s\n' "$_dim" "$_reset" "$(_oneline "$_expected")"
        printf '    %sactual:  %s %s\n' "$_dim" "$_reset" "$(_oneline "$_actual")"
    } >&2
    exit 1
}

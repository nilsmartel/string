#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# German ß uppercases to SS
check "case upper — unicode (ß → SS)" \
    "$(printf 'straße' | "$STRING" case upper)" \
    "STRASSE"

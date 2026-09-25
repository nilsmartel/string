#!/bin/sh
. "$(dirname "$0")/_harness.sh"

check "split — custom separator (comma)" \
    "$(printf 'a,b,c' | "$STRING" split ,)" \
    "$(printf 'a\nb\nc')"

#!/bin/sh
. "$(dirname "$0")/_harness.sh"

# {{ ... }} is replaced by the output of the shell command inside it
check "template — expand a shell command" \
    "$(printf 'hi {{ echo there }}' | "$STRING" template)" \
    "hi there"

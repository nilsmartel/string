#!/bin/bash

cargo build

CMD="./target/debug/string"

$CMD template <tests/template1.yaml

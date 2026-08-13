# Changelog

- 0.6.0
    - **breaking**: `contains`, `starts-with` and `ends-with` are no longer predicates over the whole
      input. Instead of exiting 0/1 they now filter the input line by line and print every matching
      line. Shell conditions relying on the exit code (`string contains foo && ...`) need to be
      rewritten, e.g. by testing the output for emptiness.
    - all three take a `--not` / `-n` flag to invert the match and print the lines that do *not* match
    - `starts-with` and `ends-with` ignore surrounding whitespace, on both the pattern and the line
- 0.5.1
    - implement `join` subcommand — joins lines with a configurable separator (inverse of `split`)
    - implement `contains`, `starts-with`, `ends-with` subcommands that exits 0/1, useful in shell if conditions. The absence means users reach for grep -q instead.
- 0.5.0
    - Merge `map` and `foreach` into simplified `each` command
    - improve documentation
- 0.4.1
    - implement `foreach` subcommand
- 0.3.6
    - implement `interleave` subcommand
- 0.3.3
    - implement `trim` subcommand
- 0.3.2
    - implement `case` subcommand for lowercase and uppercase transformations
    - implement proper testing
- 0.3.1
    - `replace` command no longer prints trailing new line
    - start cultivating changelog


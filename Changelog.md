# Changelog

- 0.6.0
    - `each --threads=N` runs up to N commands in parallel, as a threadpool. Each command's output
      is buffered and written in one piece, so parallel commands can't mangle each other's output.
      Results are printed in the order the commands finish; `each --sequential` prints them in the
      order of the input instead, waiting for earlier lines.
    - `each` draws a progress bar on stderr while more than one thread is running, leaving stdout
      free for piping
    - **breaking**: a command failing under `each` no longer aborts the run. The failure is reported
      on stderr, the remaining lines still run, and `string` exits 1 at the end.
    - errors are now printed instead of being swallowed by a silent `exit(1)`, and the output of a
      failed command is actually included in the message (it was always empty before)
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


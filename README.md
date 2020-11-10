# shell-string

Simple CLI to perform common string operations

## Usage

```
shell-string 0.1.5
Cli for common string operations. Takes input from stdin.

USAGE:
    string <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    length     Returns the length of string
    line       Pick a single line by linenumber
    replace    Replace all matching words
    split      Split up a string by a separator and print the parts on separate lines
    substr     Extract a part of a given string
```

## Installation

Given cargo is installed on your machine execute

```
cargo install shell-string
```

To verify your installation worked type `string -v` and you _should_ see the appropriate version number.

---

if you want the very latest version, check out this repository locally using

```
git clone https://github.com/nilsmartel/string
```
and build and install the code using

```
cd string   # go into the repository
cargo install --path . --force      # use force in case the binary is alread installed
```

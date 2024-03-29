![tests passing](https://github.com/nilsmartel/string/actions/workflows/rust.yml/badge.svg?branch=main)


# shell-string

Simple CLI to perform common string operations

## Usage

```
shell-string 0.4.2
Cli for common string operations. Takes input from stdin.

USAGE:
    string <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    case          Transform upper- or lowercase
    chars         Prints all chars on separate lines
    distinct      Output the set of input strings without repetitions, in order
    foreach       Applies a command to each line of input. Lines won't get applied as stdin to the command, instead
                  the command may contain the token "__var", which will get substituted with the individual lines
    help          Prints this message or the help of the given subcommand(s)
    interleave    Interleave input and only print every nth line
    length        Returns the length the input string
    line          Pick a single line by linenumber
    map           Maps each line of input to a given command. The input will be supplied as stdin of the command
    replace       Replace all matching words
    reverse       Reverse order of lines
    split         Split up a string by a separator and print the parts on separate lines
    substr        Extract a part of a given string
    template      Useful for templating, replace sections of input with the output of a shell command or script
    trim          Trim whitespace on lines and ignore empty ones
```

## Why does this exists

I'm writing ci pipelines from time to time and manipulating strings, especially templating anything, always is a HUGE pain.
Every coworker has his own style solving a problem and when it comes down to string transformation any solution not written by yourself is sheer unmaintainable.
This is mostly because there are thousands of ways to do the tasks `shell-string` does, but this cli makes them _very obvious_ and easy to understand.
More than anything I hated finding some solution for file templating over and over again. I wrote `shell-string` to never again have to think about what the best way of templating a file is.
It's always this, period.

## Why is `shell-string` good for templating files?

Because you practically have no restrictions.
You need to just drop in some environment variables? Easy, just write `{{ echo $MY_VAR }}` into the template.
Is complex logic needed? You could write `{{ console.log(crazyStuff()) }}` and you're golden. Just execute with `--shell=node`.
You want to use `haskell` in your template files? Use `--shell=ghci`!

The `string template` command is so powerful, because it doesn't do the heavy lifting itself, like a lot of alternatives do.
Instead it relies on using EVRYTHING, you could use in the terminal. You can specify, how a command get's interpreted, be it by `ghci`, `python` or `sh` (which is the default).

Using `string template` you could even set up your very own workflow for templating files. This is especially useful in CI or when configuring a fresh system.

### How does that look?
```yaml
kind: Deployment
metadata:
  name: {{ echo $GIT_REPO_NAME }}-deployment
  labels:
    deployed: "{{date}}"
    app: {{ echo $GIT_REPO_NAME }}
spec:
  replicas: {{jq .replicas < config.json}}
...
        image: {{node getImageName.js}}
...
```

Per default `sh` is used to interpret the command inside `{{`  and `}}` and, if these delimeters don't suite your style, that's okay. You can choose _any delimiter_ you fancy. And you should.

### How am using a document as a template?

give you have a document `deployment.template.yaml` and you want to derive a file called `deployment.yaml`, that's easy. Open a terminal and type
```sh
cat deployment.template.yaml | string template > deployment.yaml
```

which means
- `cat deployment.template.yaml`:   Print the file `deployment.template.yaml`
- `| string template`:              The `|` means "don't print this in a terminal, pipe it to another programm" and that programm is `string` in `template` mode.
- `> deployment.yaml`:              Write the output of this into a file called `deployment.yaml`. If the file existed, empty it beforehand.

## Installation

Given cargo is installed on your machine execute

```sh
cargo install shell-string
```

To verify your installation worked type `string --version`. Given your installation was successful you _should_ see the appropriate version number.

---

if you want the very latest version, check out this repository locally using

```sh
git clone https://github.com/nilsmartel/string
```
and build and install the code using

```sh
cd string   # go into the repository
cargo install --path . --force      # use force in case the binary is alread installed
```

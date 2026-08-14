![tests passing](https://github.com/nilsmartel/string/actions/workflows/rust.yml/badge.svg?branch=main)


# shell-string

Simple CLI to perform common string operations

## Usage
```
Cli for common string operations. Takes input from stdin.

Usage: string <COMMAND>

Commands:
  case         Transform upper- or lowercase
  reverse      Reverse order of lines
  substr       Extract part of a given string
  split        Split up a string by a separator and print the parts on separate lines
  join         Join lines with a separator into a single string
  contains     Print all lines containing the given string
  starts-with  Print all lines starting with the given prefix, ignoring leading whitespace
  ends-with    Print all lines ending with the given suffix, ignoring trailing whitespace
  length       Returns the length the input string
  replace      Replace all matching characters
  line         Pick a single line by index
  interleave   Interleave input and only print every nth line
  distinct     Output the set of input strings without repetitions, in order
  trim         Trim whitespace on lines and ignore empty ones
  chars        Prints all chars on separate lines
  template     Useful for templating, replace sections of input with the output of a shell command or script
  each         Map each line of input to a subcommand
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Why does this exists

I'm writing ci pipelines from time to time and manipulating strings, especially templating anything, always is a HUGE pain.
Every coworker has his own style solving a problem and when it comes down to string transformation any solution not written by yourself is sheer unmaintainable.
This is mostly because there are thousands of ways to do the tasks `shell-string` does, but this cli makes them _very obvious_ and easy to understand.
More than anything I hated finding some solution for file templating over and over again. I wrote `shell-string` to never again have to think about what the best way of templating a file is.
It's always this, period.

## Template Files
`shell-string` is good for templating files.

It's a very simple and clean solution where
you practically have no restrictions.
You need to just drop in some environment variables? Easy, just write `{{ echo $MY_VAR }}` into the template.
Is complex logic needed? You could write `{{ console.log(crazyStuff()) }}` and you're golden. Just execute with `--shell=node`.
You want to use `haskell` in your template files? Use `--shell=ghci`!

The `string template` command is so powerful, because it doesn't do the heavy lifting itself, like a lot of alternatives do.
Instead it relies on using EVRYTHING, you could use in the terminal. You can specify, how a command get's interpreted, be it by `ghci`, `python` or `sh` (which is the default).

Using `string template` you could even set up your very own workflow for templating files. This is especially useful in CI or when configuring a fresh system.

### Example
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

## Use It As A Threadpool

`string each` runs a command once per line of input. Waiting for those commands one after another
is pure waste whenever they sit on the network instead of the CPU, so `--threads` turns `each` into
a threadpool:

```sh
cat urls.txt | string each --threads=12 -- curl -s {}
```

Twelve requests are in flight at any moment, and `string` keeps the pool full: whenever a command
finishes, the next line is handed to the thread that just became free. Nothing is scheduled up
front, so one slow url doesn't leave eleven threads idling.

The thing that makes this usable rather than a mess is that **output is never interleaved**. The
output of a command is collected in full and written in one piece, so twelve `curl`s can't scribble
over each other halfway through a line. What you get is the same output you'd get from a sequential
run, just sooner.

### Watching it work

Because output is buffered, a long run would otherwise look like a hung terminal. So while more than
one thread is running, a progress bar is drawn — on stderr, never stdout:

```
[####################----------] 812/1200 (67%)
```

That split is deliberate: `... | string each -t 12 -- curl -s {} > results.txt` shows you the bar in
the terminal while `results.txt` receives nothing but results.

### Ordering

Results appear in the order the commands _finish_, which is what you want when you're watching them
come in. When you'd rather have them line up with your input, ask for it:

```sh
cat urls.txt | string each --threads=12 --sequential -- curl -s {}
```

`--sequential` holds finished results back until every earlier line is done, so the output matches
the input line for line. The work still runs on all twelve threads — only the printing waits.

### When something fails

One broken url shouldn't throw away the other 1199 responses. A command that exits non-zero is
reported on stderr, naming the line it came from, and the run carries on:

```
line 47: error executing command `curl`.
Process terminated with exit code exit status: 6.
Program output:
curl: (6) Could not resolve host: exmaple.com
```

At the very end `string` prints how many commands failed and exits 1, so `&&` in a script still
does the right thing — you just get all the successful output, and a full list of what went wrong,
instead of everything stopping at the first problem.

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

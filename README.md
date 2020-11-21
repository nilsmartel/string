# shell-string

Simple CLI to perform common string operations

## Usage

```
shell-string 0.2.1
Cli for common string operations. Takes input from stdin.

USAGE:
    string <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    length      Returns the length the input string
    line        Pick a single line by linenumber
    replace     Replace all matching words
    split       Split up a string by a separator and print the parts on separate lines
    substr      Extract a part of a given string
    template    Useful for templating, replace sections of input with the output of a shell command or script
```

## Why does this exists

I'm writing ci pipelines from time to time and manipulating strings, especially templating anything, always is a HUGE pain.
Every coworker has his own style solving a problem and when it comes down to string transformation any solution not written by yourself is sheer unmaintainable.
This is mostly because there are thousands of ways to do the tasks `shell-string` does, but this cli makes them _very obvious_ and easy to understand.
More than anything I hated finding some solution for file templating over and over again. I wrote `shell-string` to never again have to think about what the best way of templating a file is.
It's always this, period.

## Why is `shell-string` good for templating files?

Because you practically have no restrictions. You have the full power of any shell command at hand, even javascript if you want.
Basically you can write the stuff you have to substitute in any language you want. You can even use this to write your own substitution style of doing things. Most importantly, you don't have to.

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
```
cat deployment.template.yaml | string template > deployment.yaml
```

which means
- `cat deployment.template.yaml`:   Print the file `deployment.template.yaml`
- `| string template`:              The `|` means "don't print this in a terminal, pipe it to another programm" and that programm is `string` in `template` mode.
- `> deployment.yaml`:              Write the output of this into a file called `deployment.yaml`. If the file existed, empty it beforehand.

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

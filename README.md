## skeleton 💀

**skeleton** is a tool that makes it easier to start new projects.

It stores your commonly used project specific files in one place, letting you
easily apply them to new projects.

In essence, a `template` is a file ending in `.skeleton` with a front-matter and
content. The front-matter is structured
[YAML](https://en.wikipedia.org/wiki/YAML?useskin=vector) with effect and free
variables.

An `effect` variable is pre-defined to perform some action. As of now, there are
only 3 pre-defined variables of this type:
- `filename` (string, required) - Specifies the name of the templates destination location during
  application.
- `command` (string, optional) - A command to run on a template post-write.
- `groups` (list, optional) - Groups this template belongs to, used commonly when batch applying templates.

See [subcommand.rs](https://github.com/terror/skeleton/blob/master/src/subcommand.rs)
for further elaboration on these effect variables.

A `free` variable is used to substitute into the templates content, you can also
specify whether or not to be interactively prompted for these types of variables
when applying templates.

The binary is called `sk` and has only been tested on a Unix-based machine.

### Demo

[![asciicast](https://asciinema.org/a/rx0tWWfPTPZNXoBboE7dzX3tX.svg)](https://asciinema.org/a/rx0tWWfPTPZNXoBboE7dzX3tX)

### Installation

```
git clone https://github.com/terror/skeleton.git
cd skeleton
cargo install --path .
```

### Usage

```present cargo run -- --help
Usage: sk <COMMAND>

Commands:
  add     Add a new template
  apply   Apply a template
  edit    Edit an existing template
  list    List all templates
  remove  Remove an existing template
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Prior Art

`skeleton` is a re-implementation and improvement of the Python program I wrote a while
back called `bp`, which you can find [here](https://github.com/terror/bp).

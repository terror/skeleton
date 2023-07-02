## skel 🏗️

**skel** is a lightweight project scaffolding utility. It lets you group and use
commonly used project-specific files.

### Installation

```
git clone https://github.com/terror/skel.git
cd skel
cargo install --path .
```

### Usage

```present cargo run -- --help
Usage: skel <COMMAND>

Commands:
  add    Add a new template
  apply  Apply a template
  edit   Edit an existing template
  list   List all templates
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Prior Art

`skel` is a re-implementation and improvement of the Python program I wrote a while
back called `bp`, which you can find [here](https://github.com/terror/bp).

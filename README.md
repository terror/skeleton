## skeleton 💀

[![release](https://img.shields.io/github/release/terror/skeleton.svg?label=release&style=flat&labelColor=1d1d1d&color=424242&logo=github&logoColor=white)](https://github.com/terror/skeleton/releases/latest)
[![build](https://img.shields.io/github/actions/workflow/status/terror/skeleton/ci.yaml?branch=master&style=flat&labelColor=1d1d1d&color=424242&logo=GitHub%20Actions&logoColor=white&label=build)](https://github.com/terror/skeleton/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/terror/skeleton?style=flat&labelColor=1d1d1d&color=424242&logo=Codecov&logoColor=white)](https://codecov.io/gh/terror/skeleton)
[![downloads](https://img.shields.io/github/downloads/terror/skeleton/total.svg?style=flat&labelColor=1d1d1d&color=424242&logo=github&logoColor=white)](https://github.com/terror/skeleton/releases)

**skeleton** is a tool that makes it easier to start new projects.

It stores your commonly used project specific files in one place, letting you
easily apply them to new projects.

The binary is called `sk` and has only been tested on a Unix-based system.

## Demo

[![asciicast](https://asciinema.org/a/rx0tWWfPTPZNXoBboE7dzX3tX.svg)](https://asciinema.org/a/rx0tWWfPTPZNXoBboE7dzX3tX)

## Installation

`skeleton` should run on any unix-based system, including Linux, MacOS, and the
BSDs.

The easiest way to install it is by using
[cargo](https://doc.rust-lang.org/cargo/index.html), the Rust package manager:

```bash
cargo install skeleton-cli
```

Otherwise, see below for the complete package list:

#### Cross-platform

<table>
  <thead>
    <tr>
      <th>Package Manager</th>
      <th>Package</th>
      <th>Command</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://www.rust-lang.org>Cargo</a></td>
      <td><a href=https://crates.io/crates/skeleton>skeleton</a></td>
      <td><code>cargo install skeleton-cli</code></td>
    </tr>
    <tr>
      <td><a href=https://brew.sh>Homebrew</a></td>
      <td><a href=https://github.com/terror/homebrew-tap>terror/tap/skeleton</a></td>
      <td><code>brew install terror/tap/skeleton</code></td>
    </tr>
  </tbody>
</table>

### Pre-built binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on
[the releases page](https://github.com/terror/just-lsp/releases).

## Usage

In essence, a **template** is a file ending in `.skeleton` with a front-matter
and content. The front-matter is structured
[YAML](https://en.wikipedia.org/wiki/YAML?useskin=vector) with effect and free
variables.

An **effect** variable is pre-defined to perform some action. As of now, there
are only 3 pre-defined variables of this type:

| Name       | Type     | Required | Description                                                                   |
| ---------- | -------- | -------- | ----------------------------------------------------------------------------- |
| `command`  | String   | No       | A command to run on a template post-write.                                    |
| `filename` | String   | Yes      | Specifies the name of the templates destination location during application.  |
| `groups`   | Sequence | No       | Groups this template belongs to, used commonly when batch applying templates. |

See
[subcommand.rs](https://github.com/terror/skeleton/blob/master/src/subcommand.rs)
for further elaboration on these effect variables.

A **free** variable is used to substitute into the templates content, you can
also specify whether or not to be interactively prompted for these types of
variables when applying templates.

These types of variables follow a special kind of syntax when used within
templates, for instance:

```
---
filename: rustfmt.toml
groups: [rust-cli]
tab_spaces: 2
---
edition = "2018"
max_width = 80
newline_style = "Unix"
tab_spaces = {% tab_spaces %}
use_field_init_shorthand = true
use_try_shorthand = true
```

Note that `{% tab_spaces %}` will replace to `2` when applying this template.

For more information, consult the help output of the command-line interface:

```present cargo run -- --help
skeleton-cli 0.2.2
Liam <liam@scalzulli.com>
A project scaffolding utility

Usage: sk <COMMAND>

Commands:
  add     Add a new template
  apply   Apply a template
  edit    Edit an existing template
  list    List all templates
  remove  Remove an existing template
  rename  Rename an existing template
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Prior Art

**skeleton** is a re-implementation and improvement of the Python program I
wrote a while back called **bp**, which you can find
[here](https://github.com/terror/bp).

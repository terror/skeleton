use {
  crate::{
    engine::Engine, entry::Entry, path_ext::PathExt, search::Search,
    store::Store,
  },
  anyhow::anyhow,
  clap::Parser,
  skim::prelude::*,
  std::{collections::HashMap, fs, path::PathBuf, process, sync::Arc},
  tempdir::TempDir,
  walkdir::WalkDir,
};

mod engine;
mod entry;
mod path_ext;
mod search;
mod store;

pub(crate) const DEFAULT_TEMPLATE: &str = r#"
---
# This is a special variable that will be used as the templates
# filename when the template is used in a project.
#
# Example:
#
# filename: justfile
#
# This will create a file called `justfile` when the template is used.

filename:

# This is a variable that lets you specify what command to run on the
# file when it is used in a project.
#
# Example:
#
# command: chmod +x
#
# This will make the file executable when the template is used.

command:

# This variable lets you specify which groups this file belongs to so
# you can batch-use files in the same group.
#
# Example:
#
# groups: ["rust-cli", "utility"]
#
# This will let you use the file in a project by running either one
# of the following commands:
#
# ```
# $ skel use --groups rust-cli
# $ skel use --groups utility
# $ skel use --groups rust-cli utility
# ```

groups:

# This is a variable with a random name, you can use it within the template
# by using the `{% variable %}` syntax.

variable: foo
---
Place your content here!

Here is a variable: {% variable %}.
"#;

#[derive(Debug, Parser)]
struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  fn run(self) -> Result {
    self.subcommand.run()
  }
}

#[derive(Debug, Parser)]
enum Subcommand {
  #[clap(about = "Add a new template")]
  Add(Add),
  #[clap(about = "Edit an existing template")]
  Edit(Edit),
  #[clap(about = "Use a template")]
  Use(Use),
}

impl Subcommand {
  fn run(self) -> Result {
    match self {
      Self::Add(add) => add.run(),
      Self::Edit(edit) => edit.run(),
      Self::Use(use_) => use_.run(),
    }
  }
}

#[derive(Debug, Parser)]
struct Add {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(long, short, help = "Prepoulate the file with a template")]
  with_template: bool,
}

impl Add {
  fn run(self) -> Result {
    let store = Store::load()?;

    let editor = self
      .editor
      .or_else(|| std::env::var("EDITOR").ok())
      .ok_or_else(|| anyhow::anyhow!("Failed to locate editor"))?;

    let name = dialoguer::Input::<String>::new()
      .with_prompt("Name")
      .interact()?;

    let tempdir = TempDir::new("add")?;

    let file = tempdir.path().join(format!("{name}.skel"));

    if self.with_template {
      fs::write(&file, DEFAULT_TEMPLATE.trim_start_matches('\n'))?;
    }

    let status = process::Command::new(&editor)
      .arg(&file)
      .status()
      .expect("Failed to open temporary file in editor");

    if !status.success() {
      anyhow::bail!("Failed to open temporary file in editor");
    }

    store.write(&name, &fs::read_to_string(&file)?)?;

    Ok(())
  }
}

#[derive(Debug, Parser)]
struct Use {
  #[clap(short, long, help = "Groups to filter entries by")]
  groups: Vec<String>,
  #[clap(short, long, help = "Fuzzy search for entries with skim")]
  fuzzy: bool,
  #[clap(short, long, help = "Interactive mode")]
  interactive: bool,
}

impl Use {
  fn run(self) -> Result {
    let store = Store::load()?;
    println!("store: {:?}", store);
    Ok(())
  }
}

#[derive(Debug, Parser)]
struct Edit {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(short, long, help = "Fuzzy search for entries with skim")]
  fuzzy: bool,
}

impl Edit {
  fn run(self) -> Result {
    let store = Store::load()?;

    let editor = self
      .editor
      .or_else(|| std::env::var("EDITOR").ok())
      .ok_or_else(|| anyhow::anyhow!("Failed to locate editor"))?;

    let entries = store.entries()?;

    let entry = if self.fuzzy {
      Search::<Entry>::with(entries)
        .run()?
        .first()
        .ok_or_else(|| anyhow::anyhow!("Failed to locate entry"))?
        .clone()
    } else {
      let name = dialoguer::Input::<String>::new()
        .with_prompt("Name")
        .interact()?;

      entries
        .into_iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow!("Failed to locate entry with name: {name}"))?
    };

    let tempdir = TempDir::new("edit")?;

    let file = tempdir.path().join(format!("{}.skel", entry.name));

    fs::write(&file, entry.content)?;

    let status = process::Command::new(&editor)
      .arg(&file)
      .status()
      .expect("Failed to open temporary file in editor");

    if !status.success() {
      anyhow::bail!("Failed to open temporary file in editor");
    }

    store
      .write(&format!("{}.skel", entry.name), &fs::read_to_string(&file)?)?;

    Ok(())
  }
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}

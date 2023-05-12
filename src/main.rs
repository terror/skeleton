use {
  anyhow::anyhow,
  clap::Parser,
  skim::prelude::*,
  std::{collections::HashMap, fs, path::PathBuf, process, sync::Arc},
  tempdir::TempDir,
  walkdir::WalkDir,
};

const DEFAULT_TEMPLATE: &str = r#"
---
/*
 * This is a special variable that will be used as the templates
 * filename when the template is used in a project.
 *
 * Example:
 *
 * filename: justfile
 *
 * This will create a file called `justfile` when the template is used.
 */
filename:

/*
 * This is a variable that lets you specify what command to run on the
 * file when it is used in a project.
 *
 * Example:
 *
 * command: chmod +x
 *
 * This will make the file executable when the template is used.
 */
command:

/*
 * This variable lets you specify which groups this file belongs to so
 * you can batch-use files in the same group.
 *
 * Example:
 *
 * groups: ["rust-cli", "utility"]
 *
 * This will let you use the file in a project by running either one
 * of the following commands:
 *
 * ```
 * $ skel use --groups rust-cli
 * $ skel use --groups utility
 * $ skel use --groups rust-cli utility
 * ```
 */
groups:
---
"#;

pub(crate) struct Search<T: SkimItem + Clone> {
  items: Vec<T>,
}

impl<T: SkimItem + Clone> Search<T> {
  pub(crate) fn new(items: Vec<T>) -> Self {
    Self { items }
  }

  pub(crate) fn run(&self) -> Result<Vec<T>> {
    if self.items.len() == 1 {
      return Ok(self.items.clone());
    }

    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .build()?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    self
      .items
      .iter()
      .try_for_each(|note| tx.send(Arc::new(note.to_owned())))?;

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_else(Vec::new)
      .iter()
      .map(|selected_item| {
        (**selected_item)
          .as_any()
          .downcast_ref::<T>()
          .unwrap()
          .to_owned()
      })
      .collect::<Vec<T>>();

    match selected_items.len() {
      0 => anyhow::bail!("No templates selected"),
      _ => Ok(selected_items),
    }
  }
}

trait PathExt {
  fn create(self) -> Result<Self>
  where
    Self: Sized;
}

impl PathExt for PathBuf {
  fn create(self) -> Result<Self> {
    if !self.exists() {
      fs::create_dir_all(self.clone())?;
      Ok(self)
    } else {
      Ok(self)
    }
  }
}

#[derive(Debug, Clone)]
struct Entry {
  name: String,
  content: String,
  variables: HashMap<String, String>,
}

impl SkimItem for Entry {
  fn text(&self) -> Cow<str> {
    Cow::Owned(self.name.clone())
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(self.content.clone())
  }
}

impl TryFrom<PathBuf> for Entry {
  type Error = anyhow::Error;

  /// TODO: Implement templating engine
  fn try_from(path: PathBuf) -> Result<Self> {
    Ok(Self {
      name: path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract file stem"))?
        .to_string_lossy()
        .to_string(),
      content: fs::read_to_string(&path)?,
      variables: HashMap::new(),
    })
  }
}

#[derive(Debug, Parser)]
struct Store {
  path: PathBuf,
}

impl Store {
  fn load() -> Result<Self> {
    Ok(Self {
      path: dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?
        .join(".skel")
        .create()?,
    })
  }

  fn entries(&self) -> Result<Vec<Entry>> {
    Ok(
      WalkDir::new(&self.path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| Entry::try_from(e.into_path()))
        .collect::<Result<Vec<Entry>>>()?,
    )
  }

  fn write(self, name: &str, content: &str) -> Result {
    fs::write(self.path.join(format!("{name}.skel")), content)?;
    Ok(())
  }
}

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
      Search::<Entry>::new(entries)
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

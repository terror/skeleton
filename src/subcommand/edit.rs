use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Edit {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(short, long, help = "Fuzzy search for entries with skim")]
  fuzzy: bool,
}

impl Edit {
  pub(crate) fn run(self, store: &Store) -> Result {
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

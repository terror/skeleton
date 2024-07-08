use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Edit {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
}

impl Edit {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let editor = self
      .editor
      .or_else(|| env::var("EDITOR").ok())
      .context("Failed to locate editor")?;

    let template = Search::<Template>::with(store.templates()?.to_vec())
      .run()?
      .into_iter()
      .next()
      .context("Failed to locate template")?;

    let tempdir = TempDir::new("edit")?;

    let file = tempdir
      .path()
      .join(format!("{}{TEMPLATE_EXTENSION}", template.name()?));

    fs::write(&file, &template.content)?;

    let status = Command::new(&editor)
      .arg(&file)
      .status()
      .context("Failed to open temporary file in editor")?;

    if !status.success() {
      anyhow::bail!("Editor exited with non-zero status");
    }

    store.write(&template.name()?, &fs::read_to_string(&file)?)?;

    Ok(())
  }
}

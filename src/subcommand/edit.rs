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
      .context("failed to locate editor")?;

    let templates = Search::<Template>::with(store.templates(None)?)
      .run()
      .context("failed to search templates")?;

    for template in templates {
      let name = template.name()?;

      let tempdir = TempDir::new(&format!("edit-{name}"))?;

      let file = tempdir
        .path()
        .join(format!("{}{TEMPLATE_EXTENSION}", template.name()?));

      fs::write(&file, &template.content)?;

      let status = Command::new(&editor)
        .arg(&file)
        .status()
        .context("failed to open temporary file in editor")?;

      if !status.success() {
        bail!("editor exited with non-zero status");
      }

      store.write(&name, &fs::read_to_string(&file)?)?;

      println!("Saved changes to `{}` successfully.", name.bold());
    }

    Ok(())
  }
}

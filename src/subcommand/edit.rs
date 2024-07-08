use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Edit {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(short, long, help = "Fuzzy search for templates with skim")]
  fuzzy: bool,
}

impl Edit {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let editor = self
      .editor
      .or_else(|| env::var("EDITOR").ok())
      .context("Failed to locate editor")?;

    let templates = store.templates()?;

    let template = if self.fuzzy {
      Self::search_template(&templates)?
    } else {
      Self::prompt_template(&templates)?
    };

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

  fn search_template(templates: &[Template]) -> Result<Template> {
    Search::<Template>::with(templates.to_vec())
      .run()?
      .into_iter()
      .next()
      .context("Failed to locate template")
  }

  fn prompt_template(templates: &[Template]) -> Result<Template> {
    let name = dialoguer::Input::<String>::new()
      .with_prompt("Template name")
      .interact()?;

    templates
      .iter()
      .find(|t| t.name().map(|n| n == name).unwrap_or(false))
      .cloned()
      .context("Failed to locate template")
  }
}

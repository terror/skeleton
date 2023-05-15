use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Edit {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(short, long, help = "Fuzzy search for templates with skim")]
  fuzzy: bool,
}

impl Edit {
  pub(crate) fn run(self, store: &Store) -> Result {
    let editor = self
      .editor
      .or_else(|| std::env::var("EDITOR").ok())
      .ok_or_else(|| anyhow::anyhow!("Failed to locate editor"))?;

    let templates = store.templates()?;

    let search = || -> Result<Template> {
      Ok(
        Search::<Template>::with(templates.clone())
          .run()?
          .first()
          .ok_or_else(|| anyhow::anyhow!("Failed to locate template"))?
          .clone(),
      )
    };

    let prompt = || -> Result<Template> {
      let name = dialoguer::Input::<String>::new()
        .with_prompt("Template name")
        .interact()?;

      for template in templates.clone() {
        if template.name()? == name {
          return Ok(template);
        }
      }

      Err(anyhow::anyhow!("Failed to locate template"))
    };

    let template = match self.fuzzy {
      true => search(),
      false => prompt(),
    }?;

    let tempdir = TempDir::new("edit")?;

    let file = tempdir.path().join(format!("{}.skel", template.name()?));

    fs::write(&file, &template.content)?;

    let status = process::Command::new(editor)
      .arg(&file)
      .status()
      .expect("Failed to open temporary file in editor");

    if !status.success() {
      anyhow::bail!("Failed to open temporary file in editor");
    }

    store.write(&template.name()?, &fs::read_to_string(&file)?)?;

    Ok(())
  }
}

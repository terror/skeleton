use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Remove;

impl Remove {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let templates = store.templates()?;

    let templates = Search::<Template>::with(templates)
      .run()
      .context("Failed to locate template")?;

    for template in templates {
      fs::remove_file(template.path)?;
    }

    Ok(())
  }
}

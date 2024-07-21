use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Remove;

impl Remove {
  pub(crate) fn run(self, store: &Store) -> Result {
    let templates = store.templates(None)?;

    let templates = Search::<Template>::with(templates)
      .run()
      .context("Failed to locate template")?;

    for template in &templates {
      fs::remove_file(&template.path)?;
    }

    let names = templates
      .iter()
      .map(|template| template.name())
      .collect::<Result<Vec<_>>>()?;

    println!("Removed template(s) `{}` successfully", names.join(", "));

    Ok(())
  }
}

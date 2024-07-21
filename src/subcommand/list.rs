use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(short, long, help = "Groups to filter templates by")]
  groups: Option<Vec<String>>,
}

impl List {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let mut templates = store.templates(self.groups)?;

    templates.sort_by(|a, b| {
      a.name()
        .unwrap_or_default()
        .cmp(&b.name().unwrap_or_default())
    });

    for template in templates {
      println!("{}", template.name()?);
    }

    Ok(())
  }
}

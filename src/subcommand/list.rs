use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(short, long, help = "Groups to filter templates by")]
  groups: Option<Vec<String>>,
}

impl List {
  pub(crate) fn run(self, store: &Store) -> Result {
    let mut templates = store.templates()?;

    if let Some(filter_groups) = self.groups {
      templates.retain(|template| {
        match template
          .variables
          .get("groups")
          .cloned()
          .unwrap_or(Value::Sequence(vec![]))
          .as_sequence()
        {
          Some(groups) => groups.iter().any(|group| {
            filter_groups.contains(&group.as_str().unwrap().to_owned())
          }),
          None => false,
        }
      })
    }

    for template in templates {
      println!("{}", template.name()?);
    }

    Ok(())
  }
}

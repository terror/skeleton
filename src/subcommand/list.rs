use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(short, long, help = "Groups to filter templates by")]
  groups: Option<Vec<String>>,
}

impl List {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let templates = store.templates()?;

    let filter_groups: Option<HashSet<_>> =
      self.groups.map(|group| group.into_iter().collect());

    let mut filtered_templates = templates
      .into_iter()
      .filter(|template| {
        filter_groups.as_ref().map_or(true, |filter_group| {
          template.groups().map_or(false, |groups| {
            groups.iter().any(|group| {
              group
                .as_str()
                .map(|s| filter_group.contains(s))
                .unwrap_or(false)
            })
          })
        })
      })
      .collect::<Vec<_>>();

    filtered_templates.sort_by(|a, b| {
      a.name()
        .unwrap_or_default()
        .cmp(&b.name().unwrap_or_default())
    });

    for template in filtered_templates {
      println!("{}", template.name()?);
    }

    Ok(())
  }
}

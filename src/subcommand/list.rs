use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(short, long, help = "Groups to filter entries by")]
  groups: Option<Vec<String>>,
}

impl List {
  pub(crate) fn run(self, store: &Store) -> Result {
    let mut entries = store.entries()?;

    if let Some(filter_groups) = self.groups {
      entries = entries
        .into_iter()
        .filter(|entry| {
          match entry
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
        .collect::<Vec<_>>();
    }

    for entry in entries {
      println!("{}", entry.name);
    }

    Ok(())
  }
}

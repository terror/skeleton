use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Apply {
  #[clap(short, long, help = "Groups to filter templates by")]
  groups: Vec<String>,
  #[clap(short, long, help = "Fuzzy search for templates with skim")]
  fuzzy: bool,
  #[clap(short, long, help = "Interactive mode")]
  interactive: bool,
}

impl Apply {
  pub(crate) fn run(self, store: &Store) -> Result {
    println!("store: {:?}", store.templates()?);
    Ok(())
  }
}

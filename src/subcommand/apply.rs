use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Apply {
  #[clap(short, long, help = "Groups to filter entries by")]
  groups: Vec<String>,
  #[clap(short, long, help = "Fuzzy search for entries with skim")]
  fuzzy: bool,
  #[clap(short, long, help = "Interactive mode")]
  interactive: bool,
}

impl Apply {
  pub(crate) fn run(self, store: &Store) -> Result {
    println!("store: {:?}", store.entries()?);
    Ok(())
  }
}

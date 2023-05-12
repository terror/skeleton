use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    self.subcommand.run()
  }
}

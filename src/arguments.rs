use super::*;

#[derive(Debug, Parser)]
#[clap(
  about,
  author,
  version,
  help_template = "\
{before-help}{name} {version}
{author}
{about}

\x1b[1;4mUsage\x1b[0m: {usage}

{all-args}{after-help}
"
)]
pub(crate) struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    self.subcommand.run()
  }
}

use {
  crate::{
    arguments::Arguments, path_ext::PathExt, search::Search, store::Store,
    subcommand::Subcommand, template::Template,
  },
  clap::Parser,
  indoc::indoc,
  serde_yaml::Value,
  skim::prelude::*,
  std::{collections::HashMap, fs, path::PathBuf, process, sync::Arc},
  tempdir::TempDir,
  walkdir::WalkDir,
};

#[cfg(test)]
use crate::subcommand::DEFAULT_TEMPLATE;

mod arguments;
mod path_ext;
mod search;
mod store;
mod subcommand;
mod template;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}

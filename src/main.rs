use {
  crate::{
    arguments::Arguments, engine::Engine, entry::Entry, path_ext::PathExt,
    search::Search, store::Store, subcommand::Subcommand,
  },
  anyhow::anyhow,
  clap::Parser,
  skim::prelude::*,
  std::{collections::HashMap, fs, path::PathBuf, process, sync::Arc},
  tempdir::TempDir,
  walkdir::WalkDir,
};

mod arguments;
mod engine;
mod entry;
mod path_ext;
mod search;
mod store;
mod subcommand;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}

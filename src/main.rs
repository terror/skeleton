use {
  crate::{
    arguments::Arguments,
    path_ext::PathExt,
    search::Search,
    store::{Store, TEMPLATE_EXTENSION},
    subcommand::Subcommand,
    template::Template,
  },
  anyhow::{anyhow, Context},
  clap::Parser,
  dialoguer::Input,
  indoc::indoc,
  serde_yaml::Value,
  skim::prelude::*,
  std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::PathBuf,
    process,
    process::Command,
    sync::Arc,
  },
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

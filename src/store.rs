use super::*;

#[derive(Debug)]
pub(crate) struct Store {
  path: PathBuf,
}

impl Store {
  /// Load or create the store from the default location.
  ///
  /// The default location is `~/.skel`.
  pub(crate) fn load() -> Result<Self> {
    Ok(Self {
      path: dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?
        .join(".skel")
        .create()?,
    })
  }

  /// Returns a list of entries in the store.
  pub(crate) fn entries(&self) -> Result<Vec<Entry>> {
    WalkDir::new(&self.path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|e| e.file_type().is_file())
      .map(|e| Entry::try_from(e.into_path()))
      .collect::<Result<Vec<Entry>>>()
  }

  /// Write a new entry to the store.
  ///
  /// The entry will be written to `~/.skel/{name}.skel`.
  pub(crate) fn write(&self, name: &str, content: &str) -> Result {
    fs::write(self.path.join(format!("{name}.skel")), content)
      .map_err(|e| e.into())
  }
}

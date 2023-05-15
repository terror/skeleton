use super::*;

#[derive(Debug)]
pub(crate) struct Store {
  path: PathBuf,
}

impl Store {
  pub(crate) fn load() -> Result<Self> {
    Ok(Self {
      path: dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?
        .join(".skel")
        .create()?,
    })
  }

  pub(crate) fn exists(&self, name: &str) -> Result<bool> {
    Ok(self.templates()?.iter().any(|t| t.name().unwrap() == name))
  }

  pub(crate) fn templates(&self) -> Result<Vec<Template>> {
    WalkDir::new(&self.path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|e| e.file_type().is_file())
      .map(|e| Template::try_from(e.into_path()))
      .collect::<Result<Vec<Template>>>()
  }

  pub(crate) fn write(&self, name: &str, content: &str) -> Result {
    fs::write(self.path.join(format!("{name}.skel")), content)
      .map_err(|e| anyhow::anyhow!("Failed to write template: {}", e))
  }
}

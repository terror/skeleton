use super::*;

pub(crate) trait PathExt {
  fn create(self) -> Result<Self>
  where
    Self: Sized;
}

impl PathExt for PathBuf {
  fn create(self) -> Result<Self> {
    if self.exists() {
      return Ok(self);
    }

    fs::create_dir_all(self.clone())?;

    Ok(self)
  }
}

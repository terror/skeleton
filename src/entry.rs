use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) name: String,
  pub(crate) content: String,
  pub(crate) variables: HashMap<String, String>,
}

impl SkimItem for Entry {
  fn text(&self) -> Cow<str> {
    Cow::Owned(self.name.clone())
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(self.content.clone())
  }
}

impl TryFrom<PathBuf> for Entry {
  type Error = anyhow::Error;

  fn try_from(path: PathBuf) -> Result<Self> {
    Ok(Engine::with(path).run()?)
  }
}

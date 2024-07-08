use super::*;

pub(crate) const TEMPLATE_DIR: &str = ".skeleton";
pub(crate) const TEMPLATE_EXTENSION: &str = ".skeleton";

#[derive(Debug)]
pub(crate) struct Store {
  path: PathBuf,
}

#[cfg(test)]
impl TryFrom<PathBuf> for Store {
  type Error = anyhow::Error;

  fn try_from(path: PathBuf) -> Result<Self> {
    Ok(Self {
      path: path.join(TEMPLATE_DIR).create()?,
    })
  }
}

impl Store {
  pub(crate) fn load() -> Result<Self> {
    Ok(Self {
      path: dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to locate home directory"))?
        .join(TEMPLATE_DIR)
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
    fs::write(
      self.path.join(format!("{name}{TEMPLATE_EXTENSION}")),
      content,
    )
    .map_err(|e| anyhow!("Failed to write template: {}", e))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(crate) const DEFAULT_TEMPLATE: &str = indoc! {"
    ---
    variable: foo
    ---
    Here is a variable interpolation: {% variable %}.
  "};

  #[test]
  fn load_store_initialization() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    assert!(store.path.exists());
    assert!(store.path.is_dir());
  }

  #[test]
  fn write_and_check_existence_of_template() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "test_template";

    store.write(template_name, DEFAULT_TEMPLATE).unwrap();

    assert!(store.exists(template_name).unwrap());
  }

  #[test]
  fn list_templates_after_writing() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "test_template";

    store.write(template_name, DEFAULT_TEMPLATE).unwrap();

    let templates = store.templates().unwrap();

    assert!(templates.iter().any(|t| t.name().unwrap() == template_name));
  }

  #[test]
  fn non_existent_template() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "non_existent_template";

    assert!(!store.exists(template_name).unwrap());
  }

  #[test]
  fn read_template_content() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "test_template";
    let template_content = DEFAULT_TEMPLATE;

    store.write(template_name, template_content).unwrap();

    let templates = store.templates().unwrap();

    let template = templates
      .into_iter()
      .find(|t| t.name().unwrap() == template_name)
      .unwrap();

    assert_eq!(template.content, template_content);
  }

  #[test]
  fn overwrite_template() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "test_template";

    let updated_content = indoc! {"
      ---
      variable: bar
      ---
      Here is a variable interpolation: {% variable %}.
    "};

    store.write(template_name, DEFAULT_TEMPLATE).unwrap();
    store.write(template_name, updated_content).unwrap();

    let templates = store.templates().unwrap();

    let template = templates
      .into_iter()
      .find(|t| t.name().unwrap() == template_name)
      .unwrap();

    assert_eq!(template.content, updated_content);
  }

  #[test]
  fn delete_template() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template_name = "test_template";
    let template_content = DEFAULT_TEMPLATE;

    store.write(template_name, template_content).unwrap();

    fs::remove_file(
      store
        .path
        .join(format!("{template_name}{TEMPLATE_EXTENSION}")),
    )
    .unwrap();

    assert!(!store.exists(template_name).unwrap());
  }
}

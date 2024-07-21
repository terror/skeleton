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
        .ok_or_else(|| anyhow!("failed to locate home directory"))?
        .join(TEMPLATE_DIR)
        .create()?,
    })
  }

  pub(crate) fn exists(&self, name: &str) -> Result<bool> {
    Ok(
      self
        .templates(None)?
        .iter()
        .any(|t| t.name().unwrap() == name),
    )
  }

  /// Retrieves all templates, optionally filtered by group names.
  ///
  /// This method returns a list of all templates if no groups are specified,
  /// or a filtered list of templates that belong to at least one of the
  /// specified groups.
  ///
  /// # Arguments
  ///
  /// * `groups` - An optional vector of group names to filter the templates by.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a vector of `Template`s. If groups are specified,
  /// only templates belonging to at least one of those groups are returned.
  /// If an error occurs while fetching or filtering the templates, an `Err` is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// let store = Store::load()?;
  ///
  /// // Get all templates
  /// let all_templates = store.templates(None)?;
  ///
  /// // Get templates filtered by groups
  /// let groups = Some(vec!["web".to_string(), "backend".to_string()]);
  /// let filtered_templates = store.templates(groups)?;
  /// ```
  pub(crate) fn templates(
    &self,
    groups: Option<Vec<String>>,
  ) -> Result<Vec<Template>> {
    let all_templates = WalkDir::new(&self.path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|e| e.file_type().is_file())
      .map(|e| Template::try_from(e.into_path()))
      .collect::<Result<Vec<Template>>>()?;

    match groups {
      Some(groups) if !groups.is_empty() => Ok(
        all_templates
          .into_iter()
          .filter(|template| {
            template.groups().map_or(false, |template_groups| {
              template_groups.iter().any(|group| {
                groups.contains(&group.as_str().unwrap_or_default().to_owned())
              })
            })
          })
          .collect(),
      ),
      _ => Ok(all_templates),
    }
  }

  pub(crate) fn write(&self, name: &str, content: &str) -> Result {
    fs::write(
      self.path.join(format!("{name}{TEMPLATE_EXTENSION}")),
      content,
    )
    .map_err(|err| anyhow!("failed to write template: {err}"))
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

    let templates = store.templates(None).unwrap();

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

    let templates = store.templates(None).unwrap();

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

    let templates = store.templates(None).unwrap();

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

  #[test]
  fn filter_templates_by_group() {
    let temp_dir = TempDir::new("test").unwrap();

    let store = Store::try_from(temp_dir.into_path()).unwrap();

    let template1 = indoc! {"
      ---
      variable: foo
      groups: [web, frontend]
      ---
      Web template
    "};

    let template2 = indoc! {"
      ---
      variable: bar
      groups: [backend, api]
      ---
      Backend template
    "};

    let template3 = indoc! {"
      ---
      variable: baz
      groups: [web, backend]
      ---
      Full-stack template
    "};

    store.write("web_template", template1).unwrap();
    store.write("backend_template", template2).unwrap();
    store.write("fullstack_template", template3).unwrap();

    let web_templates = store.templates(Some(vec!["web".to_string()])).unwrap();

    assert_eq!(web_templates.len(), 2);

    assert!(web_templates
      .iter()
      .any(|t| t.name().unwrap() == "web_template"));

    assert!(web_templates
      .iter()
      .any(|t| t.name().unwrap() == "fullstack_template"));

    let api_backend_templates = store
      .templates(Some(vec!["api".to_string(), "backend".to_string()]))
      .unwrap();

    assert_eq!(api_backend_templates.len(), 2);

    assert!(api_backend_templates
      .iter()
      .any(|t| t.name().unwrap() == "backend_template"));

    assert!(api_backend_templates
      .iter()
      .any(|t| t.name().unwrap() == "fullstack_template"));

    let nonexistent_group_templates = store
      .templates(Some(vec!["nonexistent".to_string()]))
      .unwrap();

    assert_eq!(nonexistent_group_templates.len(), 0);

    let all_templates = store.templates(None).unwrap();

    assert_eq!(all_templates.len(), 3);
  }
}

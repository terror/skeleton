use super::*;

#[derive(Debug)]
pub(crate) struct Engine {
  path: PathBuf,
}

impl Engine {
  const FRONTMATTER_DELIMITER: &'static str = "---";

  pub(crate) fn with(path: PathBuf) -> Self {
    Self { path }
  }

  pub(crate) fn run(&self) -> Result<Entry> {
    let content = fs::read_to_string(&self.path)?;

    if !content.starts_with(Self::FRONTMATTER_DELIMITER) {
      anyhow::bail!(
        "Invalid template: {}, template must start with `{}` to specify its frontmatter",
        self.path.display(),
        Self::FRONTMATTER_DELIMITER
      );
    }

    let frontmatter_end = content
      .find(&format!("\n{}", Self::FRONTMATTER_DELIMITER))
      .ok_or_else(|| {
        anyhow::anyhow!(
          "Invalid template: {}, template must contain a frontmatter ending with `{}`",
          self.path.display(),
          Self::FRONTMATTER_DELIMITER
        )
      })?;

    let frontmatter =
      &content[Self::FRONTMATTER_DELIMITER.len()..frontmatter_end].trim();

    let mut variables = HashMap::new();

    if !frontmatter.is_empty() {
      variables.extend(serde_yaml::from_str::<HashMap<String, String>>(
        frontmatter,
      )?);
    }

    let content = content
      [frontmatter_end + Self::FRONTMATTER_DELIMITER.len() + 1..]
      .trim()
      .to_owned();

    if content.is_empty() {
      anyhow::bail!(
        "Invalid template: {}, file must contain content",
        self.path.display()
      );
    }

    let mut substituted_content = content;

    for (key, value) in variables.iter() {
      substituted_content =
        substituted_content.replace(&format!("{{% {} %}}", key), value);
    }

    Ok(Entry {
      name: self
        .path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract file stem"))?
        .to_string_lossy()
        .to_string(),
      content: substituted_content,
      _variables: variables,
    })
  }
}

#[cfg(test)]
mod tests {
  use {super::*, indoc::indoc};

  #[test]
  fn simple_valid_template() {
    let tempdir = TempDir::new("valid").unwrap();

    let file = tempdir.path().join("valid.skel");

    fs::write(
      &file,
      indoc! {
      "
        ---
        var: world!
        ---
        Hello, {% var %}
      ",
      },
    )
    .unwrap();

    let entry = Engine::with(file).run().unwrap();

    assert_eq!(entry.name, "valid");

    assert_eq!(entry.content, "Hello, world!");

    assert_eq!(
      entry._variables,
      HashMap::from_iter(vec![("var".to_owned(), "world!".to_owned())])
    );
  }

  #[test]
  fn default_template() {
    let tempdir = TempDir::new("default").unwrap();

    let file = tempdir.path().join("default.skel");

    fs::write(&file, DEFAULT_TEMPLATE.trim_start_matches('\n')).unwrap();

    let entry = Engine::with(file).run().unwrap();

    assert_eq!(entry.name, "default");

    assert_eq!(
      entry.content,
      "Place your content here!\n\nHere is a variable interpolation: foo."
    );

    assert_eq!(
      entry._variables,
      HashMap::from_iter(vec![
        ("command".to_owned(), "".to_owned()),
        ("filename".to_owned(), "".to_owned()),
        ("groups".to_owned(), "".to_owned()),
        ("variable".to_owned(), "foo".to_owned())
      ])
    );
  }

  #[test]
  fn invalid_frontmatter_missing_end() {
    let tempdir = TempDir::new("invalid").unwrap();

    let file = tempdir.path().join("invalid.skel");

    fs::write(
      &file,
      indoc! {
      "
        ---
        var: world!
        Hello, {% var %}
      ",
      },
    )
    .unwrap();

    let result = Engine::with(file.clone()).run();

    assert!(result.is_err());

    assert_eq!(
      result.unwrap_err().to_string(),
      format!("Invalid template: {}, template must contain a frontmatter ending with `---`", file.display())
    );
  }

  #[test]
  fn invalid_frontmatter_missing_start() {
    let tempdir = TempDir::new("invalid").unwrap();

    let file = tempdir.path().join("invalid.skel");

    fs::write(
      &file,
      indoc! {
      "
        var: world!
        ---
        Hello, {% var %}
      ",
      },
    )
    .unwrap();

    let result = Engine::with(file.clone()).run();

    assert!(result.is_err());

    assert_eq!(
      result.unwrap_err().to_string(),
      format!("Invalid template: {}, template must start with `---` to specify its frontmatter",
      file.display())
    );
  }
}

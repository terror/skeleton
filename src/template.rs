use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Template {
  pub(crate) path: PathBuf,
  pub(crate) content: String,
  pub(crate) variables: HashMap<String, Value>,
}

impl SkimItem for Template {
  fn text(&self) -> Cow<str> {
    Cow::Owned(
      self
        .path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string(),
    )
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(format!("cat \"{}\"", self.path.display()))
  }
}

impl TryFrom<PathBuf> for Template {
  type Error = anyhow::Error;

  fn try_from(path: PathBuf) -> Result<Self> {
    let content = fs::read_to_string(path.clone())?;

    if !content.starts_with(Self::FRONTMATTER_DELIMITER) {
      bail!(
        "invalid template: {}, template must start with `{}` to specify its frontmatter",
        path.display(),
        Self::FRONTMATTER_DELIMITER
      );
    }

    let frontmatter_end = content
      .find(&format!("\n{}", Self::FRONTMATTER_DELIMITER))
      .ok_or_else(|| {
        anyhow!(
          "invalid template: {}, template must contain a frontmatter ending with `{}`",
          path.display(),
          Self::FRONTMATTER_DELIMITER
        )
      })?;

    let frontmatter =
      &content[Self::FRONTMATTER_DELIMITER.len()..frontmatter_end].trim();

    let mut variables = HashMap::new();

    if !frontmatter.is_empty() {
      variables
        .extend(serde_yaml::from_str::<HashMap<String, Value>>(frontmatter)?);
    }

    Ok(Template {
      path,
      content,
      variables,
    })
  }
}

impl Template {
  const FRONTMATTER_DELIMITER: &'static str = "---";

  pub(crate) fn name(&self) -> Result<String> {
    self
      .path
      .file_stem()
      .ok_or_else(|| anyhow!("failed to get template name"))
      .and_then(|s| {
        s.to_str()
          .ok_or_else(|| anyhow!("failed to convert template name"))
          .map(|s| s.to_owned())
      })
  }

  pub(crate) fn filename(&self) -> Option<serde_yaml::Value> {
    self.variables.get("filename").cloned()
  }

  pub(crate) fn command(&self) -> Option<serde_yaml::Value> {
    self.variables.get("command").cloned()
  }

  pub(crate) fn groups(&self) -> Option<serde_yaml::Sequence> {
    self
      .variables
      .get("groups")
      .cloned()
      .unwrap_or(Value::Sequence(vec![]))
      .as_sequence()
      .cloned()
  }

  pub(crate) fn replace_variable(&mut self, variable: &str, value: Value) {
    self.variables.insert(variable.to_owned(), value);
  }

  pub(crate) fn substitute(&self) -> Result<String> {
    let frontmatter_end = self.content
    .find(&format!("\n{}", Self::FRONTMATTER_DELIMITER))
    .ok_or_else(|| {
      anyhow!(
        "invalid template: {}, template must contain a frontmatter ending with `{}`",
        self.path.display(),
        Self::FRONTMATTER_DELIMITER
      )
    })?;

    let content = self.content
      [frontmatter_end + Self::FRONTMATTER_DELIMITER.len() + 1..]
      .trim()
      .to_owned();

    if content.is_empty() {
      bail!(
        "invalid template: {}, file must contain content",
        self.path.display()
      );
    }

    let mut substituted_content = content;

    for (key, value) in self.variables.iter() {
      substituted_content = substituted_content.replace(
        &format!("{{% {} %}}", key),
        serde_yaml::to_string(value)?.trim(),
      );
    }

    Ok(substituted_content)
  }
}

#[cfg(test)]
mod tests {
  use {super::*, indoc::indoc};

  #[test]
  fn simple_valid_template() {
    let tempdir = TempDir::new("valid").unwrap();

    let file = tempdir.path().join("valid.skeleton");

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

    let template = Template::try_from(file).unwrap();

    assert_eq!(template.name().unwrap(), "valid");

    assert_eq!(template.substitute().unwrap(), "Hello, world!");

    assert_eq!(
      template.variables,
      HashMap::from_iter(vec![(
        "var".to_owned(),
        Value::String("world!".to_owned())
      )])
    );
  }

  #[test]
  fn default_template() {
    let tempdir = TempDir::new("default").unwrap();

    let file = tempdir.path().join("default.skeleton");

    fs::write(&file, DEFAULT_TEMPLATE.trim_start_matches('\n')).unwrap();

    let template = Template::try_from(file).unwrap();

    assert_eq!(template.name().unwrap(), "default");

    assert_eq!(
      template.substitute().unwrap(),
      "Place your content here!\n\nHere is a variable interpolation: foo."
    );

    assert_eq!(
      template.variables,
      HashMap::from_iter(vec![
        ("command".to_owned(), Value::Null),
        ("filename".to_owned(), Value::Null),
        ("groups".to_owned(), Value::Null),
        ("variable".to_owned(), Value::String("foo".to_owned()))
      ])
    );
  }

  #[test]
  fn populated_variables() {
    let tempdir = TempDir::new("default").unwrap();

    let file = tempdir.path().join("default.skeleton");

    let template = indoc! {
      "
        ---
        command: chmod +x
        filename: justfile
        groups: [\"rust-cli\", \"utility\"]
        variable: bar
        ---
        Place your content here!

        Here is a variable interpolation: {% variable %}.
      "
    };

    fs::write(&file, template).unwrap();

    let template = Template::try_from(file).unwrap();

    assert_eq!(template.name().unwrap(), "default");

    assert_eq!(
      template.substitute().unwrap(),
      "Place your content here!\n\nHere is a variable interpolation: bar."
    );

    assert_eq!(
      template.variables,
      HashMap::from_iter(vec![
        ("command".to_owned(), Value::String("chmod +x".to_owned())),
        ("filename".to_owned(), Value::String("justfile".to_owned())),
        (
          "groups".to_owned(),
          Value::Sequence(vec![
            Value::String("rust-cli".to_owned()),
            Value::String("utility".to_owned())
          ])
        ),
        ("variable".to_owned(), Value::String("bar".to_owned()))
      ])
    );
  }

  #[test]
  fn invalid_frontmatter_missing_end() {
    let tempdir = TempDir::new("invalid").unwrap();

    let file = tempdir.path().join("invalid.skeleton");

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

    let result = Template::try_from(file.clone());

    assert!(result.is_err());

    assert_eq!(
      result.unwrap_err().to_string(),
      format!("invalid template: {}, template must contain a frontmatter ending with `---`", file.display())
    );
  }

  #[test]
  fn invalid_frontmatter_missing_start() {
    let tempdir = TempDir::new("invalid").unwrap();

    let file = tempdir.path().join("invalid.skeleton");

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

    let result = Template::try_from(file.clone());

    assert!(result.is_err());

    assert_eq!(
      result.unwrap_err().to_string(),
      format!("invalid template: {}, template must start with `---` to specify its frontmatter",
      file.display())
    );
  }

  #[test]
  fn replace_variable() {
    let tempdir = TempDir::new("replace").unwrap();

    let file = tempdir.path().join("replace.skeleton");

    fs::write(
      &file,
      indoc! {
      "
      ---
      greeting: hello
      name: world
      ---
      {% greeting %}, {% name %}!
    ",
      },
    )
    .unwrap();

    let mut template = Template::try_from(file).unwrap();

    assert_eq!(template.name().unwrap(), "replace");
    assert_eq!(template.substitute().unwrap(), "hello, world!");

    template.replace_variable("greeting", Value::String("welcome".to_owned()));
    assert_eq!(template.substitute().unwrap(), "welcome, world!");

    template.replace_variable("name", Value::String("friend".to_owned()));
    assert_eq!(template.substitute().unwrap(), "welcome, friend!");

    template.replace_variable("extra", Value::String("unused".to_owned()));

    assert_eq!(
      template.variables,
      HashMap::from_iter(vec![
        ("greeting".to_owned(), Value::String("welcome".to_owned())),
        ("name".to_owned(), Value::String("friend".to_owned())),
        ("extra".to_owned(), Value::String("unused".to_owned()))
      ])
    );

    assert_eq!(template.substitute().unwrap(), "welcome, friend!");
  }
}

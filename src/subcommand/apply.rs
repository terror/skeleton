use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Apply {
  #[clap(short, long, help = "Groups to filter templates by")]
  groups: Option<Vec<String>>,
  #[clap(short, long, help = "Interactive mode")]
  interactive: bool,
  #[clap(short, long, help = "Overwrite existing files")]
  overwrite: bool,
}

impl Apply {
  pub(crate) fn run(self, store: &Store) -> Result<()> {
    let templates = store.templates()?;

    let filter_groups: Option<HashSet<_>> =
      self.groups.map(|groups| groups.into_iter().collect());

    let filtered_templates: Vec<_> = templates
      .into_iter()
      .filter(|template| {
        filter_groups.as_ref().map_or(true, |groups| {
          template.groups().map_or(false, |template_groups| {
            template_groups
              .iter()
              .any(|group| groups.contains(group.as_str().unwrap()))
          })
        })
      })
      .collect();

    let mut templates = Search::<Template>::with(filtered_templates)
      .run()
      .context("Failed to locate template")?;

    let effect_variables = ["filename", "command", "groups"];

    for template in &mut templates {
      let name = template.name()?;

      let filename = template.filename().ok_or_else(|| {
        anyhow::anyhow!("Template `{}` does not specify a filename", name)
      })?;

      println!("Applying template `{}`", name);

      let free_variables: Vec<_> = template
        .variables
        .keys()
        .filter(|k| !effect_variables.contains(&k.as_str()))
        .cloned()
        .collect();

      if self.interactive {
        let theme = dialoguer::theme::ColorfulTheme::default();

        for variable in free_variables {
          template.replace_variable(
            &variable,
            serde_yaml::to_value(
              Input::<String>::with_theme(&theme)
                .with_prompt(format!("Enter value for {}", variable))
                .interact_text()?,
            )?,
          );
        }
      }

      let command = template.command().ok_or_else(|| {
        anyhow::anyhow!("Template `{}` does not specify a command", name)
      })?;

      let file_path = std::env::current_dir()?.join(filename.as_str().unwrap());

      if file_path.exists() && !self.overwrite {
        println!(
          "File `{}` already exists. Skipping template `{}`.",
          file_path.display(),
          name
        );
        continue;
      }

      if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
          format!("Failed to create directories for `{}`", file_path.display())
        })?;
      }

      let mut content = template.substitute()?;

      if !content.ends_with('\n') {
        content.push('\n');
      }

      fs::write(&file_path, content).with_context(|| {
        format!("Failed to write file `{}`", file_path.display())
      })?;

      let mut command_parts = command.as_str().unwrap().split_whitespace();

      let command_name = command_parts.next().unwrap();
      let command_args: Vec<_> = command_parts.collect();

      let output = Command::new(command_name)
        .args(command_args)
        .arg(&file_path)
        .output()
        .with_context(|| {
          format!("Failed to execute command: {}", command_name)
        })?;

      if !output.status.success() {
        anyhow::bail!(
          "Command failed for template `{}`: {}",
          name,
          String::from_utf8_lossy(&output.stderr)
        );
      }
    }

    Ok(())
  }
}

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
    let mut templates = Search::<Template>::with(store.templates(self.groups)?)
      .run()
      .context("failed to locate template")?;

    let effect_variables = ["filename", "command", "groups"];

    for template in &mut templates {
      let name = template.name()?;

      let filename = template.filename().ok_or_else(|| {
        anyhow!("template `{}` does not specify a filename", name.bold())
      })?;

      let free_variables = template
        .variables
        .keys()
        .filter(|k| !effect_variables.contains(&k.as_str()))
        .cloned()
        .collect::<Vec<_>>();

      if self.interactive {
        let theme = ColorfulTheme::default();

        for variable in free_variables {
          template.replace_variable(
            &variable,
            serde_yaml::to_value(
              Input::<String>::with_theme(&theme)
                .with_prompt(format!("Enter value for `{}`", variable.bold()))
                .interact_text()?,
            )?,
          );
        }
      }

      let file_path =
        std::env::current_dir()?.join(filename.as_str().unwrap_or_default());

      if file_path.exists() && !self.overwrite {
        println!(
          "File `{}` already exists, specify `--overwrite` to overwrite it",
          file_path.display()
        );

        continue;
      }

      if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).context(anyhow!(
          "failed to create directories for `{}`",
          file_path.display()
        ))?;
      }

      let mut content = template.substitute()?;

      if !content.ends_with('\n') {
        content.push('\n');
      }

      fs::write(&file_path, content)
        .context(anyhow!("failed to write file `{}`", file_path.display()))?;

      println!("Applied template `{name}` to `{}`", file_path.display());

      if let Some(command) = template.command() {
        let mut command_parts =
          command.as_str().unwrap_or_default().split_whitespace();

        let command_name = command_parts
          .next()
          .ok_or(anyhow!("command for template `{}` is empty", name.bold()))?;

        let command_args: Vec<_> = command_parts.collect();

        let output = Command::new(command_name)
          .args(command_args)
          .arg(&file_path)
          .output()
          .context(format!("failed to execute command: {command_name}"))?;

        if !output.status.success() {
          bail!(
            "command failed for template `{}`: {}",
            name,
            String::from_utf8_lossy(&output.stderr)
          );
        }
      }
    }

    Ok(())
  }
}

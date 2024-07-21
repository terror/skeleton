use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Add {
  #[clap(long, short, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(long, short, help = "Pre-populate the file with a template")]
  with_template: bool,
  #[clap(long, short)]
  from_file: Option<PathBuf>,
}

impl Add {
  pub(crate) fn run(self, store: &Store) -> Result {
    let editor = self
      .editor
      .or_else(|| env::var("EDITOR").ok())
      .ok_or_else(|| anyhow!("failed to locate editor"))?;

    let mut name = Input::<String>::new()
      .with_prompt("Template name")
      .interact()?;

    while store.exists(&name)? {
      println!(
        "A template with that name already exists, please choose another name."
      );

      name = Input::<String>::new()
        .with_prompt("Template name")
        .interact()?;
    }

    let tempdir = TempDir::new("add")?;

    let file = tempdir.path().join(format!("{name}{TEMPLATE_EXTENSION}"));

    if self.with_template {
      fs::write(&file, DEFAULT_TEMPLATE.trim_start_matches('\n'))?;
    }

    if let Some(filename) = self.from_file {
      if self.with_template {
        println!("Noticed `--with-template` specified, overriding default template with file");
      }

      fs::write(
        &file,
        format!(
          "---\nfilename: {}\n---\n{}",
          filename.display(),
          fs::read_to_string(&filename)?
        ),
      )?;
    }

    let status = process::Command::new(editor)
      .arg(&file)
      .status()
      .context("failed to open temporary file in editor")?;

    if !status.success() {
      bail!("editor exited with non-zero status");
    }

    store.write(&name, &fs::read_to_string(&file)?)?;

    println!("Template `{}` added successfully.", name.bold());

    Ok(())
  }
}

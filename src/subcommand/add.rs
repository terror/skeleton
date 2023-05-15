use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Add {
  #[clap(short, long, help = "Editor to edit the file with")]
  editor: Option<String>,
  #[clap(long, short, help = "Prepopulate the file with a template")]
  with_template: bool,
}

impl Add {
  pub(crate) fn run(self, store: &Store) -> Result {
    let editor = self
      .editor
      .or_else(|| std::env::var("EDITOR").ok())
      .ok_or_else(|| anyhow::anyhow!("Failed to locate editor"))?;

    let mut name = dialoguer::Input::<String>::new()
      .with_prompt("Template name")
      .interact()?;

    while store.exists(&name)? {
      println!(
        "A template with that name already exists, please choose another name."
      );

      name = dialoguer::Input::<String>::new()
        .with_prompt("Template name")
        .with_initial_text(&name)
        .interact()?;
    }

    let tempdir = TempDir::new("add")?;

    let file = tempdir.path().join(format!("{name}.skel"));

    if self.with_template {
      fs::write(&file, DEFAULT_TEMPLATE.trim_start_matches('\n'))?;
    }

    let status = process::Command::new(editor)
      .arg(&file)
      .status()
      .expect("Failed to open temporary file in editor");

    if !status.success() {
      anyhow::bail!("Failed to open temporary file in editor");
    }

    store.write(&name, &fs::read_to_string(&file)?)?;

    Ok(())
  }
}

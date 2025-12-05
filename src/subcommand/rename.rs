use super::*;

pub(crate) fn run(store: &Store) -> Result {
  let templates = store.templates(None)?;

  let templates = Search::<Template>::with(templates)
    .run()
    .context("Failed to locate template")?;

  for template in &templates {
    let old_name = template.name()?;

    let mut new_name = Input::<String>::new()
      .with_prompt(format!("New name for template `{}`", old_name.bold()))
      .interact()?;

    while store.exists(&new_name)? && new_name != old_name {
      println!(
        "A template with that name already exists, please choose another name."
      );

      let input = Input::<String>::new()
        .with_prompt(format!("New name for template `{}`", old_name.bold()))
        .interact()?;

      if input == new_name {
        println!("Skipping rename of template `{old_name}`");
        break;
      }

      new_name = input;
    }

    if new_name != old_name && !store.exists(&new_name)? {
      let content = fs::read_to_string(&template.path)?;
      store.write(&new_name, &content)?;
      fs::remove_file(&template.path)?;
      println!(
        "Renamed template `{}` to `{}` successfully",
        old_name.bold(),
        new_name.bold()
      );
    }
  }

  Ok(())
}

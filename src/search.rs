use super::*;

pub(crate) struct Search<T: SkimItem + Clone> {
  items: Vec<T>,
}

impl<T: SkimItem + Clone> Search<T> {
  pub(crate) fn with(items: Vec<T>) -> Self {
    Self { items }
  }

  pub(crate) fn run(&self) -> Result<Vec<T>> {
    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .build()?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    self
      .items
      .iter()
      .try_for_each(|note| tx.send(Arc::new(note.to_owned())))?;

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_else(Vec::new)
      .iter()
      .map(|selected_item| {
        (**selected_item)
          .as_any()
          .downcast_ref::<T>()
          .unwrap()
          .to_owned()
      })
      .collect::<Vec<T>>();

    match selected_items.len() {
      0 => anyhow::bail!("No templates selected"),
      _ => Ok(selected_items),
    }
  }
}

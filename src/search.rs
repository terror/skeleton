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
      .try_for_each(|item| tx.send(Arc::new(item.to_owned())))?;

    drop(tx);

    let output = Skim::run_with(&options, Some(rx));

    let selected_items = output
      .and_then(|out| {
        if out.is_abort {
          None
        } else {
          Some(out.selected_items)
        }
      })
      .unwrap_or_else(Vec::new);

    let selected_items = selected_items
      .iter()
      .map(|selected_item| {
        (**selected_item)
          .as_any()
          .downcast_ref::<T>()
          .unwrap()
          .to_owned()
      })
      .collect::<Vec<T>>();

    Ok(selected_items)
  }
}

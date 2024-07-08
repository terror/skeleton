use {
  super::*,
  crate::subcommand::{
    add::Add, apply::Apply, edit::Edit, list::List, remove::Remove,
  },
};

mod add;
mod apply;
mod edit;
mod list;
mod remove;

pub(crate) const DEFAULT_TEMPLATE: &str = indoc! {"
  ---
  # This is a special variable that will be used as the templates
  # filename when the template is used in a project.
  #
  # Example:
  #
  # filename: justfile
  #
  # This will create a file called `justfile` when the template is applied.

  filename:

  # This is a variable that lets you specify what command to run on the
  # file when it is applied in a project.
  #
  # Example:
  #
  # command: chmod +x
  #
  # This will make the file executable when the template is applied.

  command:

  # This variable lets you specify which groups this file belongs to so
  # you can batch-apply files in the same group.
  #
  # Example:
  #
  # groups: [\"rust-cli\", \"utility\"]
  #
  # This will let you use the file in a project by running either one
  # of the following commands:
  #
  # ```
  # $ skel apply --groups rust-cli
  # $ skel apply --groups utility
  # $ skel apply --groups rust-cli utility
  # ```

  groups:

  # This is a variable with a random name, you can use it within the template
  # by using the `{% variable %}` syntax.

  variable: foo
  ---
  Place your content here!

  Here is a variable interpolation: {% variable %}.
"};

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[clap(about = "Add a new template")]
  Add(Add),
  #[clap(about = "Apply a template")]
  Apply(Apply),
  #[clap(about = "Edit an existing template")]
  Edit(Edit),
  #[clap(about = "List all templates")]
  List(List),
  #[clap(about = "Remove an existing template")]
  Remove(Remove),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    let store = Store::load()?;

    match self {
      Self::Add(add) => add.run(&store),
      Self::Apply(apply) => apply.run(&store),
      Self::Edit(edit) => edit.run(&store),
      Self::List(list) => list.run(&store),
      Self::Remove(remove) => remove.run(&store),
    }
  }
}

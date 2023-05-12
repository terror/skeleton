use {
  super::*,
  crate::subcommand::{add::Add, apply::Apply, edit::Edit},
};

mod add;
mod apply;
mod edit;

pub(crate) const DEFAULT_TEMPLATE: &str = r#"
---
# This is a special variable that will be used as the templates
# filename when the template is used in a project.
#
# Example:
#
# filename: justfile
#
# This will create a file called `justfile` when the template is used.

filename:

# This is a variable that lets you specify what command to run on the
# file when it is used in a project.
#
# Example:
#
# command: chmod +x
#
# This will make the file executable when the template is used.

command:

# This variable lets you specify which groups this file belongs to so
# you can batch-use files in the same group.
#
# Example:
#
# groups: ["rust-cli", "utility"]
#
# This will let you use the file in a project by running either one
# of the following commands:
#
# ```
# $ skel use --groups rust-cli
# $ skel use --groups utility
# $ skel use --groups rust-cli utility
# ```

groups:

# This is a variable with a random name, you can use it within the template
# by using the `{% variable %}` syntax.

variable: foo
---
Place your content here!

Here is a variable: {% variable %}.
"#;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[clap(about = "Add a new template")]
  Add(Add),
  #[clap(about = "Apply a template")]
  Apply(Apply),
  #[clap(about = "Edit an existing template")]
  Edit(Edit),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Add(add) => add.run(),
      Self::Apply(apply) => apply.run(),
      Self::Edit(edit) => edit.run(),
    }
  }
}

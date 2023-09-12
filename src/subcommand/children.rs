use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Children {
}

impl Children {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;
    index.update()?;

    index.get_children()?;

    Ok(Box::new(Empty {}))
  }
}

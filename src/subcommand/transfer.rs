use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Transfer {
  #[clap(long, help = "Delete the whole transfer log table.")]
  delete: bool,
  #[clap(long, help = "Delete transfer logs for blocks before height <TRIM>.")]
  trim: Option<u64>,
}

impl Transfer {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;
    index.update()?;

    if self.delete && self.trim.is_some() {
      return Err(anyhow!("Cannot use both --delete and --trim"));
    }

    if self.delete {
      println!("deleting transfer log table");
      index.delete_transfer_log()?;
      return Ok(Box::new(Empty {}));
    }

    if self.trim.is_some() {
      let trim = self.trim.unwrap();
      println!("deleting transfer logs for blocks before {trim}");
      index.trim_transfer_log(trim)?;
    }

    let (rows, first_key, last_key) = index.show_transfer_log_stats()?;
    if rows == 0 {
      println!("the transfer table has {rows} rows");
    } else {
      println!(
        "the transfer table has {rows} rows from height {} to height {}",
        first_key.unwrap(),
        last_key.unwrap()
      );
    }

    Ok(Box::new(Empty {}))
  }
}

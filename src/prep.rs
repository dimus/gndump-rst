use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
  pub mysql_url: Url,
  pub mysql_dir: PathBuf,
  pub pg_dir: PathBuf,
  pub batch_size: i64,
}

impl Config {
  pub fn new() -> Config {
    // error handling is probably overkill here,
    // learning...
    dotenv::dotenv().ok();
    let mysql_dir = env::var("MYSQL_PATH").expect("no env var MYSQL_PATH");
    let pg_dir = env::var("PG_PATH").expect("no env var PG_PATH");
    let mysql_url = env::var("DATABASE_URL").expect("no env var DATABASE_URL");
    let batch_size = env::var("BATCH_SIZE").expect("no env var BATCH_SIZE");
    let batch_size: i64 = batch_size
      .parse()
      .expect("BATCH_SIZE does not convert to int");

    let mut conf = Config {
      mysql_url: Url::parse(&mysql_url).unwrap(),
      mysql_dir: PathBuf::new(),
      pg_dir: PathBuf::new(),
      batch_size,
    };
    conf.pg_dir.push(Path::new(&pg_dir));
    conf.mysql_dir.push(Path::new(&mysql_dir));
    conf
  }
}

pub fn set_dirs(pg_dir: &Path, mysql_dir: &Path) -> Result<(), io::Error> {
  remove_dirs(pg_dir, mysql_dir)?;
  for path in &[pg_dir, mysql_dir] {
    fs::create_dir_all(path)?;
  }
  Ok(())
}

fn remove_dirs<'a>(pg_dir: &'a Path, mysql_dir: &'a Path) -> Result<(), io::Error> {
  for path in &[pg_dir, mysql_dir] {
    if !path.exists() {
      continue;
    };
    fs::remove_dir_all(path)?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn remove_dirs_test() {
    let pg_dir = &Path::new("/tmp/test/pg");
    let mysql_dir = &Path::new("/tmp/test/mysql");
    assert_eq!(remove_dirs(pg_dir, mysql_dir).is_err(), false);
    assert_eq!(pg_dir.exists(), false);
    assert_eq!(mysql_dir.exists(), false);
  }

  #[test]
  fn set_dirs_test() {
    let pg_dir = &Path::new("/tmp/test/pg");
    let mysql_dir = &Path::new("/tmp/test/mysql");
    assert_eq!(set_dirs(pg_dir, mysql_dir).is_err(), false);
    assert_eq!(pg_dir.is_dir(), true);
    assert_eq!(mysql_dir.is_dir(), true);
  }
}

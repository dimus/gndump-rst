use crate::models::{
  DataSource, NameString, NameStringIndex, VernacularString, VernacularStringIndex,
};
use crate::prep::Config;
use csv::Writer;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::thread;

pub struct Dumper {
  pub mysql_path: PathBuf,
  pub pg_path: PathBuf,
  pub db: MysqlConnection,
  pub batch_size: i64,
}

impl Dumper {
  pub fn new(conf: Config) -> Dumper {
    let db_url = conf.mysql_url.as_str();
    let db =
      MysqlConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url));
    Dumper {
      mysql_path: conf.mysql_dir,
      pg_path: conf.pg_dir,
      db,
      batch_size: conf.batch_size,
    }
  }
}

pub fn mysql_to_csv(conf: Config) -> Result<(), Box<dyn Error>> {
  let conf2 = conf.clone();
  let ns = thread::spawn(move || {
    dump_name_strings(conf2.clone()).unwrap();
    dump_data_sources(conf2.clone()).unwrap();
    dump_vernacular_strings(conf2.clone()).unwrap();
    dump_vernacular_string_indices(conf2.clone()).unwrap();
  });
  let nsi = thread::spawn(move || {
    dump_name_string_indices(conf).unwrap();
  });
  ns.join().unwrap();
  nsi.join().unwrap();
  Ok(())
}

fn dump_data_sources(conf: Config) -> Result<(), Box<dyn Error>> {
  let dmp = Dumper::new(conf);
  info!("Dumping data_sources table...");
  let dss = DataSource::all(&dmp.db);
  let mut path = dmp.mysql_path.clone();
  path.push("data_sources.csv");
  let mut wtr = Writer::from_path(&path)?;
  for ds in dss {
    let id = format!("{}", ds.id);
    let title = ds.title.unwrap_or(String::new());
    let desc = ds.description.unwrap_or(String::new());
    let updated_at = match ds.updated_at {
      Some(date) => date.format("%Y-%m-%d").to_string(),
      None => String::new(),
    };
    wtr.write_record(&[id, title, desc, updated_at])?;
    wtr.flush()?;
  }
  Ok(())
}

fn dump_name_strings(conf: Config) -> Result<(), Box<dyn Error>> {
  let dmp = Dumper::new(conf);
  info!("Dumping name_strings table...");
  let mut path = dmp.mysql_path.clone();
  path.push("name_strings.csv");
  let mut wtr = Writer::from_path(&path)?;

  let mut count: i64 = 0;
  loop {
    let dss = NameString::select(&dmp.db, dmp.batch_size, dmp.batch_size * count);
    if dss.len() == 0 {
      break;
    };
    for ds in dss {
      let id = format!("{}", ds.id);
      let name = ds.name.unwrap_or(String::new());
      let name: String = name.as_str().replace("\u{0000}", "");

      wtr.write_record(&[id, name])?;
      wtr.flush()?;
    }
    count += 1;
  }
  Ok(())
}

fn dump_name_string_indices(conf: Config) -> Result<(), Box<dyn Error>> {
  info!("Dumping name_string_indices table...");
  let dmp = Dumper::new(conf);
  let mut path = dmp.mysql_path.clone();
  path.push("name_string_indices.csv");
  let mut wtr = Writer::from_path(&path)?;

  let mut count: i64 = 0;
  loop {
    let dss = NameStringIndex::select(&dmp.db, dmp.batch_size, dmp.batch_size * count);
    if dss.len() == 0 {
      break;
    };
    for ds in dss {
      let data_source_id = format!("{}", ds.data_source_id);
      let name_string_id = format!("{}", ds.name_string_id);
      let taxon_id = ds.taxon_id;
      let global_id = ds.global_id.unwrap_or(String::new());
      let url = ds.url.unwrap_or(String::new());
      let rank = ds.rank.unwrap_or(String::new());
      let accepted_taxon_id = ds.accepted_taxon_id.unwrap_or(String::new());
      let classification_path = ds.classification_path.unwrap_or(String::new());
      let classification_path_ids = ds.classification_path_ids.unwrap_or(String::new());
      let created_at = match ds.created_at {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => String::new(),
      };
      let updated_at = match ds.created_at {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => String::new(),
      };
      let nomenclatural_code_id = match ds.nomenclatural_code_id {
        Some(id) => format!("{}", id),
        None => String::new(),
      };
      let local_id = ds.local_id.unwrap_or(String::new());
      let classification_path_ranks = ds.classification_path_ranks.unwrap_or(String::new());
      wtr.write_record(&[
        data_source_id,
        name_string_id,
        taxon_id,
        global_id,
        url,
        rank,
        accepted_taxon_id,
        classification_path,
        classification_path_ids,
        created_at,
        updated_at,
        nomenclatural_code_id,
        local_id,
        classification_path_ranks,
      ])?;
      wtr.flush()?;
    }
    count += 1;
  }
  Ok(())
}

fn dump_vernacular_strings(conf: Config) -> Result<(), Box<dyn Error>> {
  let dmp = Dumper::new(conf);
  info!("Dumping vernacular_strings table...");
  let dss = VernacularString::all(&dmp.db);
  let mut path = dmp.mysql_path.clone();
  path.push("vernacular_strings.csv");
  let mut wtr = Writer::from_path(&path)?;
  for ds in dss {
    let id = format!("{}", ds.id);
    let name = ds.name.unwrap_or(String::new());
    wtr.write_record(&[id, name])?;
    wtr.flush()?;
  }
  Ok(())
}

fn dump_vernacular_string_indices(conf: Config) -> Result<(), Box<dyn Error>> {
  let dmp = Dumper::new(conf);
  info!("Dumping vernacular_string_indices table...");
  let dss = VernacularStringIndex::all(&dmp.db);
  let mut path = dmp.mysql_path.clone();
  path.push("vernacular_string_indices.csv");
  let mut wtr = Writer::from_path(&path)?;
  for ds in dss {
    let data_source_id = format!("{}", ds.data_source_id);
    let vernacular_string_id = format!("{}", ds.vernacular_string_id);
    let taxon_id = ds.taxon_id;
    let language = ds.language.unwrap_or(String::new());
    let locality = ds.locality.unwrap_or(String::new());
    let country_code = ds.country_code.unwrap_or(String::new());
    wtr.write_record(&[
      data_source_id,
      vernacular_string_id,
      taxon_id,
      language,
      locality,
      country_code,
    ])?;
    wtr.flush()?;
  }
  Ok(())
}

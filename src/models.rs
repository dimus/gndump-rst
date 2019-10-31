use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use super::schema::data_sources;
use super::schema::data_sources::dsl::data_sources as all_data_sources;

#[derive(Queryable, Debug)]
pub struct DataSource {
  pub id: i32,
  pub title: Option<String>,
  pub description: Option<String>,
}

impl DataSource {
  pub fn show(id: i32, conn: &MysqlConnection) -> Vec<DataSource> {
    all_data_sources
      .find(id)
      .load::<DataSource>(conn)
      .expect("Error loding data_source")
  }

  pub fn all(conn: &MysqlConnection) -> Vec<DataSource> {
    all_data_sources
      .order(data_sources::id.desc())
      .load::<DataSource>(conn)
      .expect("Error loading data_sources")
  }
}

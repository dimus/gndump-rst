use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::data_sources::dsl::data_sources;
use crate::schema::name_string_indices::dsl::name_string_indices;
use crate::schema::name_strings::dsl::name_strings;
use crate::schema::vernacular_string_indices::dsl::vernacular_string_indices;
use crate::schema::vernacular_strings::dsl::vernacular_strings;

#[derive(Queryable, Debug)]
pub struct DataSource {
  pub id: i32,
  pub title: Option<String>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}

impl DataSource {
  pub fn all(conn: &MysqlConnection) -> Vec<DataSource> {
    data_sources
      .load::<DataSource>(conn)
      .expect("Error loading data_sources")
  }
}

#[derive(Queryable, Debug)]
pub struct NameString {
  pub id: i32,
  pub name: Option<String>,
}

impl NameString {
  pub fn select(conn: &MysqlConnection, limit: i64, offset: i64) -> Vec<NameString> {
    name_strings
      .limit(limit)
      .offset(offset)
      .load::<NameString>(conn)
      .expect("Error loading data_sources")
  }
}

#[derive(Queryable, Debug)]
pub struct NameStringIndex {
  pub data_source_id: i32,
  pub name_string_id: i32,
  pub taxon_id: String,
  pub global_id: Option<String>,
  pub url: Option<String>,
  pub rank: Option<String>,
  pub accepted_taxon_id: Option<String>,
  pub classification_path: Option<String>,
  pub classification_path_ids: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
  pub nomenclatural_code_id: Option<i32>,
  pub local_id: Option<String>,
  pub classification_path_ranks: Option<String>,
}

impl NameStringIndex {
  pub fn select(conn: &MysqlConnection, limit: i64, offset: i64) -> Vec<NameStringIndex> {
    name_string_indices
      .limit(limit)
      .offset(offset)
      .load::<NameStringIndex>(conn)
      .expect("Error loading data_sources")
  }
}

#[derive(Queryable, Debug)]
pub struct VernacularString {
  pub id: i32,
  pub name: Option<String>,
}

impl VernacularString {
  pub fn all(conn: &MysqlConnection) -> Vec<VernacularString> {
    vernacular_strings
      .load::<VernacularString>(conn)
      .expect("Error loading data_sources")
  }
}

#[derive(Queryable, Debug)]
pub struct VernacularStringIndex {
  pub data_source_id: i32,
  pub vernacular_string_id: i32,
  pub taxon_id: String,
  pub language: Option<String>,
  pub locality: Option<String>,
  pub country_code: Option<String>,
}

impl VernacularStringIndex {
  pub fn all(conn: &MysqlConnection) -> Vec<VernacularStringIndex> {
    vernacular_string_indices
      .load::<VernacularStringIndex>(conn)
      .expect("Error loading data_sources")
  }
}

use std::{collections::HashMap, str};
use anyhow::Error;
use rusqlite::types::ValueRef;
use crate::log;

pub struct MetapowerSqlite3 {
    pub db_name: String,
}

impl Default for MetapowerSqlite3 {
    fn default() -> Self {
        Self::new()
    }
}

impl MetapowerSqlite3 {
    pub fn new() -> MetapowerSqlite3 {
        MetapowerSqlite3 {
            db_name: "population_database".to_string(),
        }
    }
    pub fn create_table(&self, table_sql: String) -> Result<&MetapowerSqlite3, Error> {
        // Establish connection to database, it creates the file if it doesn't exist
        let conn = ic_sqlite::CONN.lock().unwrap();

        conn.execute(
            &table_sql,
            [],
        )?;

        log!("Tables created successfully.");

        Ok(self)
    }
    pub fn update_table(&self, table_sql: String) -> Result<&MetapowerSqlite3, Error> {
        // Establish connection to database, it creates the file if it doesn't exist
        let conn = ic_sqlite::CONN.lock().unwrap();

        conn.execute(
            &table_sql,
            [],
        )?;

        log!("Tables created successfully.");

        Ok(self)
    }
    pub fn insert_record(&self, sql: &str, parameters: &[&dyn rusqlite::ToSql]) -> Result<i64, Error> {
        let conn = ic_sqlite::CONN.lock().unwrap();

        // Insert a record
        let _ = conn.execute(
            sql,
            parameters
        //    params![id, name],
        )?;

        let last_id = conn.last_insert_rowid();

        log!("A record has been inserted successfully.");
    
        Ok(last_id)
    }
    pub fn query_db(sql: &str, columns: Vec<&str>) -> Result<Vec<HashMap::<String, String>>, Error> {
        log!("sql: {}", sql);
        let conn = ic_sqlite::CONN.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let mut values = HashMap::<String, String>::new();
            for col_name_ref in columns.iter() {
                let col_name = *col_name_ref;
                match row.get_ref(col_name) {
                    Ok(ValueRef::Text(value)) => {
                        let value_str = str::from_utf8(value)?;
                        values.insert(col_name.to_string(), value_str.to_string());
                    }
                    Ok(ValueRef::Integer(value)) => {
                        values.insert(col_name.to_string(), value.to_string());
                    }
                    Ok(ValueRef::Real(value)) => {
                        values.insert(col_name.to_string(), value.to_string());
                    }
                    _ => {}
                }
            }

            Ok(values)
        })?;
    
        let mut records = vec![];
        for row in rows {
            records.push(row?);
        }
    
        Ok(records)
    }
}


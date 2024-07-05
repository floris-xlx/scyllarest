use crate::ScyllaClient;
use scylla::IntoTypedRows;
use scylla::QueryResult;
use scylla::{Session, SessionBuilder};
use serde_json::{json, Value};
use std::error::Error;

impl ScyllaClient {
    pub async fn new(known_nodes: Vec<&str>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut builder = SessionBuilder::new();

        for node in known_nodes {
            builder = builder.known_node(node);
        }

        let session: Session = builder.build().await?;

        Ok(ScyllaClient { session })
    }

    // pub fn query(&self, keyspace: &str, table: &str) -> QueryBuilder<'_> {
    //     QueryBuilder::new(Operations::Select, keyspace, table, self)
    // }

    pub async fn get_table_columns(
        &self,
        keyspace: &str,
        table: &str,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let query: String = format!(
            "SELECT column_name, kind, type FROM system_schema.columns WHERE keyspace_name = '{}' AND table_name = '{}'",
            keyspace, table
        );

        let result: QueryResult = self.session.query(query, ()).await?;
        let rows: Vec<scylla::frame::response::result::Row> = result.rows.ok_or("No rows found")?;

        let mut columns: Vec<Value> = vec![];
        for row in rows.into_typed::<(String, String, String)>() {
            let (column_name, kind, data_type) = row?;
            columns.push(json!({
                "column_name": column_name,
                "kind": kind,
                "data_type": data_type,
            }));
        }

        let json_result: Value = json!({
            "keyspace": keyspace,
            "table": table,
            "columns": columns,
        });

        Ok(json_result)
    }
}

pub mod keyspaces;
pub mod query;
pub mod tables;
pub mod columns;
pub mod methods;
pub mod client;
pub mod materialized_view;
pub mod index;
pub mod auth;

use scylla::Session;


#[derive(Debug)]
pub struct ScyllaClient {
    pub session: Session,
}

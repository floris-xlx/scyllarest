use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;

use scyllarest::ScyllaClient;
use serde_json::Value;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    println!("Starting server on port {}", port);

    HttpServer::new(|| App::new().service(hello).service(rest_root))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/rest/v1/{keyspace}/{table}")]
async fn rest_root(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (keyspace, table) = path.into_inner();

    // auth to scylladb
    let client_auth_state = match auth_client_scylladb().await {
        Ok(_) => {
            println!("Auth success");
            "success"
        }
        Err(e) => {
            println!("Auth failed: {}", e);
            "failed"
        }
    };

    // get table columns and return as json
    let columns: Value = match getting_table_columns(
        &keyspace,
        &table,
    ).await {
        Ok(columns) => {
            println!("\x1b[32mGetting table columns success\x1b[0m");
            columns
        }
        Err(e) => {
            println!("\x1b[31mGetting table columns failed: {}\x1b[0m", e);
            serde_json::Value::Null
        }
    };

    // get the columns key from the json
    let column_values = columns.get("columns").unwrap();


    let response: serde_json::Value = serde_json::json!({
        "route": "/rest/v1",
        "keyspace": keyspace,
        "table": table,
        "client_auth_state": client_auth_state,
        "table_columns": column_values,
        "ssl": "true"
    });
    Ok(HttpResponse::Ok().json(response))
}

pub async fn auth_client_scylladb() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    Ok(())
}

async fn getting_table_columns(
    keyspace: &str,
    table: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let columns: serde_json::Value = client
        .get_table_columns(keyspace, table)
        .await?;

    Ok(columns)
}

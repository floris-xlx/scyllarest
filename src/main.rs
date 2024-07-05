use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder, Result };
use dotenv::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = std::env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap();
    println!("Starting server on port {}", port);

    HttpServer::new(|| { 
        App::new()
        .service(hello) 
        .service(rest_root)
    })

        .bind(("127.0.0.1", port))?
        .run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[get("/rest/v1/{keyspace}/{table}")]
async fn rest_root(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (
        keyspace,
        table
    ) = path.into_inner();
    
    let response: serde_json::Value = serde_json::json!({
        "route": "/rest/v1",
        "keyspace": keyspace,
        "table": table
    });

    Ok(HttpResponse::Ok().json(response))
}

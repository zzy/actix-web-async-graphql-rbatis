mod gql;
mod dbs;
mod users;

use actix_web::{guard, web, App, HttpServer};

use crate::gql::{build_schema, graphql, graphiql};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema().await;

    println!("GraphQL UI: http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(
                web::resource("/graphiql").guard(guard::Get()).to(graphiql),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

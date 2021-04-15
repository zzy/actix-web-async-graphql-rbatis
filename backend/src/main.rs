mod util;
mod gql;
mod dbs;
mod users;

use actix_web::{guard, web, App, HttpServer};

use crate::util::constant::CFG;
use crate::gql::{build_schema, graphql, graphiql};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema().await;

    println!(
        "GraphQL UI: http://{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    );

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource(CFG.get("GQL_VER").unwrap())
                    .guard(guard::Post())
                    .to(graphql),
            )
            .service(
                web::resource(CFG.get("GIQL_VER").unwrap())
                    .guard(guard::Get())
                    .to(graphiql),
            )
    })
    .bind(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))?
    .run()
    .await
}

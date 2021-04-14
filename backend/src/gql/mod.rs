pub mod mutations;
pub mod queries;

use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

use crate::dbs::mysql::my_pool;
use crate::gql::queries::QueryRoot;

type ActixSchema = Schema<
    queries::QueryRoot,
    async_graphql::EmptyMutation,
    async_graphql::EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    // 获取 mongodb datasource 后，可以将其增加到：
    // 1. 作为 async-graphql 的全局数据；
    // 2. 作为 Tide 的应用状态 State；
    // 3. 使用 lazy-static.rs
    let my_pool = my_pool().await;

    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mongodb datasource  in the schema object.
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(my_pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql"),
        ),
    ))
}

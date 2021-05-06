pub mod mutations;
pub mod queries;

use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

use crate::util::constant::CFG;
use crate::dbs::mysql::my_pool;
use crate::gql::{queries::QueryRoot, mutations::MutationRoot};

type ActixSchema = Schema<
    queries::QueryRoot,
    mutations::MutationRoot,
    async_graphql::EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    // 获取 mysql 数据池后，可以将其增加到：
    // 1. 作为 async-graphql 的全局数据；
    // 2. 作为 actix-web 的应用程序数据，优势是可以进行原子操作；
    // 3. 使用 lazy-static.rs
    let my_pool = my_pool().await;

    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mysql pool  in the schema object.
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(my_pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            GraphQLPlaygroundConfig::new(CFG.get("GQL_VER").unwrap())
                .subscription_endpoint(CFG.get("GQL_VER").unwrap()),
        ),
    ))
}

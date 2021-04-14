use rbatis::core::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;

pub const MYSQL_URL: &'static str =
    "mysql://root:mysql@localhost:3306/budshome";

pub async fn my_pool() -> Rbatis {
    let rb = Rbatis::new();

    let mut opts = DBPoolOptions::new();
    opts.max_connections = 100;

    rb.link_opt(MYSQL_URL, &opts).await.unwrap();

    rb
}

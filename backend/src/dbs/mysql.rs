use rbatis::core::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;

use crate::util::constant::CFG;

pub async fn my_pool() -> Rbatis {
    let rb = Rbatis::new();

    let mut opts = DBPoolOptions::new();
    opts.max_connections = 100;

    rb.link_opt(CFG.get("MYSQL_URI").unwrap(), &opts).await.unwrap();

    rb
}

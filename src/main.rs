use crate::path::path;
use dotenv::dotenv;
use sqlx::PgPool;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware,Origin};
use chrono::prelude::*;


mod path;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();


    let now  = Local::now();
    println!("Waktu Sekarang : {} ", now.to_string());

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")
            .expect("DB Config Err"))
            .await.expect("DB Connection Err");
    let cors = CorsMiddleware::new()
            .allow_methods(
                "GET, PUT, DELETE, POST, OPTIONS"
                .parse::<HeaderValue>()
                .unwrap(),
            )
            .allow_origin(Origin::from("*"))
            .allow_credentials(false);
    tide::log::start();
    let mut app = tide::with_state(pool.clone());   
    app.with(cors);
    path(&mut app);
    app.listen("0.0.0.0:9000").await?;
    Ok(())

}
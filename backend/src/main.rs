use backend::run;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL has to be set in .env");

    run(database_url).await;
}

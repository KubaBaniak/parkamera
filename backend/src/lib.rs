mod routes;

use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_url: String) {
    let app = create_routes();
    let db = Database::connect(database_url).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

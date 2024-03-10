mod entities;
mod routes;

use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_url: String) {
    let db = Database::connect(database_url).await.unwrap();
    let app = create_routes(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use salvo::prelude::*;

mod routes;
pub mod router;
pub mod db;

const ADDRESS: &str = "127.0.0.1:7878";

#[tokio::main]
async fn main() {
    let user_router = router::user();

    
    println!("Starting server...");

    println!("Connecting to database with name '{}'...", db::get_database().await.unwrap().name());

    println!("Server running at http://{}", ADDRESS);
    Server::new(TcpListener::bind(ADDRESS)).serve(user_router).await;
}
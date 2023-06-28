use dotenvy::dotenv;
use tonic::{transport::Server};
use pb::restaurant_service_server::RestaurantServiceServer;

mod db;
mod handlers;

pub mod pb {
    tonic::include_proto!("restaurant");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let pool = db::create_connection_pool().await;

    let addr = "127.0.0.1:50051".parse()?;
    let restaurant = handlers::RestaurantHandler::new(pool);

    Server::builder()
        .add_service(RestaurantServiceServer::new(restaurant))
        .serve(addr)
        .await?;

    Ok(())
}

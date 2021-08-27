mod websocket;
mod routes;
mod graphql;

#[tokio::main]
async fn main() {
    routes::server().await;
    websocket::websocket().await;
}
use axum::routing::get;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Serialize)]
struct MessageOut {
    text: String,
    user: String,
    time: chrono::DateTime<chrono::Utc>,
}

async fn on_connect(socket: SocketRef) {
    info!("Connecting to socket {:?}", socket);

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Connected to {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on("message", |socket: SocketRef, Data::<MessageIn>(data)| {
        info!("Received message {:?}", data);

        let response = MessageOut {
            text: format!("{} bro", data.text),
            user: format!("user-{}", socket.id),
            time: chrono::Utc::now(),
        };

        let _ = socket.within(data.room).emit("message", &response);
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer)
        );

    info!("So, so disappointing");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

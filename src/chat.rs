use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

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

#[tracing::instrument]
pub async fn on_connect(socket: SocketRef) {
    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Connected to socket:{:?} room:{:?}", socket, room);
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
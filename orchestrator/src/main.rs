mod grpc_stream;
mod kafka_producer;
mod solver_auction;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use solver_auction::{run_auction, Quote};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedIntent {
    pub id: Uuid,
    pub owner: String,
    pub session: String,
    pub pair: String,
    pub side: String,
    pub size: u64,
    pub limit_price: u64,
    pub max_slippage_bps: u16,
    pub expires_slot: u64,
    pub signature: String,
}

#[derive(Debug, Serialize)]
struct Health {
    status: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("nexus_orchestrator=info,tower_http=info")
        .init();

    tokio::spawn(grpc_stream::run());

    let app = Router::new()
        .route("/health", get(|| async { Json(Health { status: "ok" }) }))
        .route("/ws/intents", get(intent_ws))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("orchestrator listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn intent_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(message)) = receiver.next().await {
        let Message::Text(raw) = message else {
            continue;
        };

        let intent = match serde_json::from_str::<SignedIntent>(&raw) {
            Ok(intent) => intent,
            Err(err) => {
                let _ = sender
                    .send(Message::Text(format!(
                        r#"{{"error":"bad_intent","detail":"{err}"}}"#
                    )))
                    .await;
                continue;
            }
        };

        kafka_producer::publish_intent(&intent).await;
        let quote = run_auction(&intent).await;
        if let Err(err) = send_quote(&mut sender, &quote).await {
            warn!("failed to send quote: {err}");
            break;
        }
    }
}

async fn send_quote(
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    quote: &Quote,
) -> anyhow::Result<()> {
    sender
        .send(Message::Text(serde_json::to_string(quote)?))
        .await?;
    Ok(())
}

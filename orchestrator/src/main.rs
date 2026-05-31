mod grpc_stream;
mod kafka_producer;
mod solver_auction;
mod ui;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solver_auction::{quote_for_step, Quote};
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
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
        .route("/", get(ui::index))
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
        let accepted = json!({
            "event": "intent_accepted",
            "intent_id": intent.id,
            "owner": intent.owner,
            "pair": intent.pair,
            "side": intent.side,
            "size": intent.size,
            "limit_price": intent.limit_price
        });

        if let Err(err) = send_json(&mut sender, &accepted).await {
            warn!("failed to send quote: {err}");
            break;
        }

        for step in 1..=5 {
            sleep(Duration::from_millis(300)).await;
            let quote = quote_for_step(&intent, step);
            if let Err(err) = send_quote(&mut sender, &quote).await {
                warn!("failed to send quote: {err}");
                break;
            }
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

async fn send_json(
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    value: &serde_json::Value,
) -> anyhow::Result<()> {
    sender.send(Message::Text(value.to_string())).await?;
    Ok(())
}

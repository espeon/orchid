use axum::{
    extract::{ConnectInfo, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use axum_extra::TypedHeader;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::debug;
use twitch::chat::setup_twitch_chat;
use ws::{WebsocketCollection, WebsocketHandler, WsMessage};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;
pub mod err;
pub mod twitch;
pub mod ws;

#[derive(Clone)]
pub struct AppState {
    ws_collection: Arc<Mutex<WebsocketCollection>>,
    sub_manager: Arc<Mutex<twitch::chat::manager::SubscriptionManager>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // setup sub manager
    let sub_manager = twitch::chat::manager::SubscriptionManager::new();

    let ws_collection = Arc::new(Mutex::new(WebsocketCollection::new(sub_manager.clone())));

    // Setup twitch chat
    let cloned_ws_collection = ws_collection.clone();
    let cloned_sub_manager = sub_manager.clone();
    let twitch_chat_task = tokio::spawn(async move {
        setup_twitch_chat(ws_collection, cloned_sub_manager).await;
    });

    let state = AppState {
        ws_collection: cloned_ws_collection,
        sub_manager,
    };

    println!("Ok!");

    // build our application with some routes
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", any(handle_ws))
        .route("/broadcast", get(broadcast_message))
        .route("/global_sub", get(global_sub))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(state);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    tokio::select! {
        //_ = serve_task => {},
        _ = twitch_chat_task => {},
    }
}

async fn handle_ws(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let wsh = WebsocketHandler::new(ws, user_agent, addr);
    wsh.ws_upgrade(addr.to_string(), state.ws_collection).await
}

#[derive(Deserialize)]
struct BroadcastMessageQuery {
    message: String,
}

async fn broadcast_message(
    Query(query): Query<BroadcastMessageQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let message = WsMessage::Text(query.message);
    debug!("Broadcasting message: {:?}", message);
    let _ = state
        .ws_collection
        .lock()
        .await
        .broadcast_message(message)
        .await;
    StatusCode::OK
}

#[derive(Deserialize)]
struct GlobalSubscriptionQuery {
    username: String,
}

async fn global_sub(
    Query(query): Query<GlobalSubscriptionQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut mgr = state.sub_manager.lock().await;
    mgr.subscribe(query.username, "global".to_string())
        .await
        .unwrap();
    StatusCode::OK
}

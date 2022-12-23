use libcradle::protocol;

use axum::{
    // extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing;

struct ProtocolServer {
    name: String,
    songs: Vec<String>,
    audio_server: Arc<Mutex<AudioServer>>,
}

impl ProtocolServer {
    pub fn new(name: String, audio_server: Arc<Mutex<AudioServer>>) -> ProtocolServer {
        ProtocolServer {
            name: name,
            songs: vec![],
            audio_server: audio_server,
        }
    }
    pub fn enqueue(&mut self, payload: protocol::Enqueue) -> protocol::EnqueueResponse {
        println!("Got song: {:?}", &payload.filepath);
        tracing::debug!("Got enqueue POST");
        tracing::debug!("Got song: {:?}", &payload.filepath);
        self.songs.push(payload.filepath);

        protocol::EnqueueResponse { success: true }
    }
}

async fn enqueue(
    payload: Json<protocol::Enqueue>,
    state: Arc<Mutex<ProtocolServer>>,
) -> impl IntoResponse {
    let mut s = state.lock().unwrap();
    let result = s.enqueue(payload.0);
    (StatusCode::OK, Json(result))
}

struct AudioServer {
    current_song: String,
}

impl AudioServer {
    pub fn new() -> AudioServer {
        AudioServer {
            current_song: "Nothing playing".to_string(),
        }
    }
    pub fn play(&mut self, song: String) -> () {
        println!("Currently playing: {:?}", self.current_song);
        println!("Starting: {:?}", song);
        self.current_song = song;
    }
}

fn start_protocol_server() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // initialise our server structure
            let audio_server = Arc::new(Mutex::new(AudioServer::new()));
            let protocol_server = Arc::new(Mutex::new(ProtocolServer::new(
                "cradle".to_string(),
                Arc::clone(&audio_server),
            )));

            // initialize tracing
            tracing_subscriber::fmt::init();

            // build our application with a route
            let app = Router::new().route(
                "/enqueue",
                post({
                    let shared_state = Arc::clone(&protocol_server);
                    move |body| enqueue(body, shared_state)
                }),
            );
            // Start the audio server

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        })
}

fn main() {
    start_protocol_server();
}

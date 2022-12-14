use libcradle::protocol;

use std::sync::{Arc, Mutex};
use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use tracing;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

struct ServerState {
    name: String,
    songs: Vec<String>,
}

impl ServerState {
    pub fn new(name: String) -> ServerState {
        ServerState {
            name: name, 
            songs: vec![],
        }
    }
    pub fn enqueue(&mut self, payload: Json<protocol::Enqueue>) -> impl IntoResponse {
        tracing::debug!("Got enqueue POST");
        tracing::debug!("Got song: {:?}", &payload.0.filepath);
        self.songs.push(payload.0.filepath);
        
        let resp = protocol::EnqueueResponse {
            success: true,
        };
        (StatusCode::OK, Json(resp))
    }
}


async fn enqueue(
    payload: Json<protocol::Enqueue>,
    state: Arc<Mutex<ServerState>>
) -> impl IntoResponse {
    let mut s = state.lock().unwrap();
    s.enqueue(payload)
}


#[tokio::main]
async fn main() {
    // initialise our server structure
    let shared_server_state = Arc::new(Mutex::new(ServerState::new("cradle".to_string())));
     // initialize tracing
     tracing_subscriber::fmt::init();

     // build our application with a route
     let app = Router::new()
        .route("/enqueue", post({
            let shared_state = Arc::clone(&shared_server_state);
            move |body| enqueue(body, shared_state)
        }),
    );
 
     // run our app with hyper
     // `axum::Server` is a re-export of `hyper::Server`
     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
     tracing::debug!("listening on {}", addr);
     axum::Server::bind(&addr)
         .serve(app.into_make_service())
         .await
         .unwrap();

}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Enqueue {
    pub filepath: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnqueueResponse {
    pub success: bool,
}
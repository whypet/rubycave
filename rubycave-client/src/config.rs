use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub fov: f32,
    pub sensitivity: f32,
}

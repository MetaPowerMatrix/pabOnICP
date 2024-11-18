use serde::Serialize;

#[derive(Serialize)]
pub struct PathInfo {
    pub absolute_path: String,
    pub saved_name: String,
}
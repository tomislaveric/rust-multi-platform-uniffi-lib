use crate::models::types::Character;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CharacterResponse {
    pub results: Vec<Character>,
}

#[derive(thiserror::Error, uniffi::Error, Debug, PartialEq)]
pub enum RequestError {
    #[error("Failed to complete request")]
    Failed,
}

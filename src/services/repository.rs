use std::sync::Arc;
use serde::Deserialize;
use crate::models::types::{Character, RequestError};
use crate::models::responses::CharacterResponse;

#[derive(uniffi::Object, Deserialize, Debug, PartialEq)]
pub struct Repository {}
#[uniffi::export]
impl Repository {
    #[uniffi::constructor]
    fn new() -> Arc<Self> {
        Self {  }.into()
    }
    
    pub async fn get_character(&self, id: u8) -> Result<Arc<Character>, RequestError> {

        let result = ureq::get(format!("https://rickandmortyapi.com/api/character/{id}").as_str()).call();
        match result {
            Ok(res) => match res.into_json() {
                Ok(c) => Ok(Arc::new(c)),
                Err(_) => Err(RequestError::Failed),
            },
            Err(_) => Err(RequestError::Failed),
        }
    }

    pub async fn get_characters(&self, page: u8) -> Result<Vec<Arc<Character>>, RequestError> {

        let result = ureq::get(format!("https://rickandmortyapi.com/api/character?page={page}").as_str()).call();
        match result {
            Ok(res) => match res.into_json::<CharacterResponse>() {
                Ok(json) => Ok(json.results.into_iter().map(|element| Arc::new(element)).collect()),
                Err(_) => Err(RequestError::Failed),
            },
            Err(_) => Err(RequestError::Failed),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::repository::Repository;
    use crate::models::types::Character;
    use std::sync::Arc;

    #[async_std::test]
    async fn get_single_character() {
        let sut = Repository {};
        let result = sut.get_character(1).await.unwrap();
        let expected_char = Character {
            id: 1,
            name: "Rick Sanchez".to_string(),
            species: "Human".to_string(),
            image: "https://rickandmortyapi.com/api/character/avatar/1.jpeg".to_string(),
        };
        let expected = Arc::new(expected_char);
        assert_eq!(result, expected);
    }
    
    #[async_std::test]
    async fn get_all_characters_test() {
        
        let sut = Repository {};
        let result = sut.get_characters(1).await.unwrap();
        assert_eq!(result.len(), 20)
    }
}

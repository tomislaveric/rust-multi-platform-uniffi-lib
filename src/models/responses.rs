   use serde::Deserialize;
   use crate::models::types::Character;
   
   #[derive(Deserialize)]
   pub struct CharacterResponse {
      pub results: Vec<Character>
   }   

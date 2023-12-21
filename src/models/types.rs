use serde::Deserialize;
#[derive(uniffi::Object, Deserialize, Debug, PartialEq)]
pub struct Character {
    pub id: u8,
    pub name: String,
    pub species: String,
    pub image: String,
}

#[uniffi::export]
impl Character {
    fn id(&self) -> u8 {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn species(&self) -> String {
        self.species.clone()
    }
    fn image(&self) -> String {
        self.image.clone()
    }
}

#[derive(thiserror::Error, uniffi::Error, Debug, PartialEq)]
pub enum RequestError {
    #[error("Failed to complete request")]
    Failed,
}
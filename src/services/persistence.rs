use crate::models::types::Note;

#[derive(Debug)]
struct Persistence {
    saved_notes: Vec<Note>,
}

impl Persistence {
    pub async fn save_note(&mut self, id: &u8, text: &str) {
        let note = Note {
            id: *id,
            text: text.to_string(),
        };
        self.saved_notes.push(note);
    }

    pub async fn get_note(&self, id: &u8) -> &Note {
        let note = self.saved_notes.iter().find(|note| note.id == *id);
        match note {
            Some(expr) => expr,
            None => panic!(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::types::Note;
    use crate::services::persistence::Persistence;

    #[async_std::test]
    async fn get_note_test() {
        let expected_note = Note {
            id: 123,
            text: "some text".to_string(),
        };
        let sut = Persistence {
            saved_notes: vec![expected_note],
        };

        let note = sut.get_note(&123).await;
        assert_eq!(note.text, "some text");
    }

    #[async_std::test]
    async fn save_note_test() {
        let mut sut = Persistence {
            saved_notes: vec![],
        };
        sut.save_note(&123, "some text").await;

        assert_eq!(sut.saved_notes.len(), 1);
    }
}

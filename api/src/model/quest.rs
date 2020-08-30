use chrono::{prelude::Local, DateTime};
use uuid::Uuid;

/// Quest domain model.
/// Represents a task to be done, a wish to accomplish or something to acquire.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Quest {
    id: Uuid,
    text: String,
    complete: bool,
    creation_date: DateTime<Local>,
    last_update_date: DateTime<Local>,
}

impl<'a> Quest {
    /// Returns a `Quest` with the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text of the quest.
    pub fn new(text: String) -> Quest {
        Quest {
            id: Uuid::new_v4(),
            text: text,
            complete: false,
            creation_date: Local::now(),
            last_update_date: Local::now(),
        }
    }

    /// Read model

    /// Returns the UUID of a `Quest`.
    pub fn id(&'a self) -> &'a Uuid {
        &self.id
    }

    /// Returns the text of a `Quest`.
    #[allow(dead_code)]
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Write model

    /// Updates a `Quest` with a new text.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the new text of the quest.
    pub fn update(&mut self, text: String) -> Result<(), String> {
        self.text = text;
        self.last_update_date = Local::now();
        Ok(())
    }

    /// Consume model

    /// Consumes the `Quest` and returns its UUID.
    pub fn into_id(self) -> Uuid {
        self.id
    }

    /// Consumes the `Quest` and returns its text.
    pub fn into_text(self) -> String {
        self.text
    }
}

/// Impl `Default` constructor to use `Quest` as a DTO.
/// It can then be used to deserialize incomplete JSON objets.
impl Default for Quest {
    fn default() -> Quest {
        Quest::new(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let q = Quest::new("foo".to_string());
        assert_ne!(uuid::Uuid::nil(), q.id);
        assert_eq!("foo", q.text);
        assert_eq!(false, q.complete);
    }

    // Write model

    #[test]
    fn valid_update() -> Result<(), String> {
        let mut q = Quest::new("foo".to_string());
        q.update("bar".to_string())?;
        assert_eq!("bar", q.text);
        Ok(())
    }
}

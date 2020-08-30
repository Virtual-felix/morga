use std::collections::HashMap;

use crate::model::quest::Quest;

pub struct Service {
    quests: HashMap<uuid::Uuid, Quest>,
}

pub fn new() -> Service {
    Service { quests: HashMap::new() }
}

impl Service {
    pub fn create(&mut self, text: String) -> Result<Quest, String> {
        let quest = Quest::new(text);
        self.quests.insert(quest.id().clone(), quest.clone());
        Ok(quest)
    }

    pub fn get(&self, quest: Quest) -> Result<Quest, String> {
        let q = self.quests.get(&quest.into_id());
        match q {
            Some(q) => Ok(q.clone()),
            None => Err("get: quest not found".to_string()),
        }
    }

    pub fn list(&self) -> Result<HashMap<uuid::Uuid, Quest>, String> {
        let quests = self.quests.clone();
        Ok(quests)
    }

    pub fn update(&mut self, quest: Quest) -> Result<Quest, String> {
        let q = self.quests.get_mut(quest.id());
        match q {
            Some(q) => {
                q.update(quest.into_text())?;
                Ok(q.clone())
            }
            None => Err("list: quest not found".to_string()),
        }
    }

    pub fn delete(&mut self, quest: Quest) -> Result<bool, String> {
        let q = self.quests.remove(&quest.into_id());
        match q {
            Some(_) => Ok(true),
            None => Err("delete: quest not found".to_string()),
        }
    }
}

// This service is left untested as it will change a lot with the introduction of the database very soon.

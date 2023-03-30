use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The Todo struct is the principal object in our app.
/// It contains all the information about the todo.
///
/// It can be serialized and deserialized with serde.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    id: Uuid,
    title: String,
    done: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            title,
            done: false,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let name = String::from("Terminar esta app");
        let todo = Todo::new(name.clone());
        assert_eq!(todo.get_title(), name);
        assert!(!todo.is_done());
    }
}

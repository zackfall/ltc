use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// This struct will store all the Todos that will be made
/// by the user
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todos(Vec<Todo>);

impl From<Vec<Todo>> for Todos {
    /// Get a Todos type from a Vector o Todo
    fn from(value: Vec<Todo>) -> Self {
        Todos(value)
    }
}

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

    /// Get the Todo id
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    /// Get the Todo title
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Tell us if the Todo is done
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

    #[test]
    fn test_new_todos() {
        let vector = vec![
            String::from("Terminar esta app"),
            String::from("Segunda Tarea"),
            String::from("Tercera Tarea"),
        ];

        let mut todos_vec: Vec<Todo> = Vec::new();
        for todo in &vector {
            todos_vec.push(Todo::new(todo.to_owned()));
        }

        let todos: Todos = Todos::from(todos_vec);

        for todo in todos.0.iter() {
            assert!(!todo.get_id().is_empty());
            assert!(!todo.get_title().is_empty());
            assert!(!todo.is_done());
        }
    }
}

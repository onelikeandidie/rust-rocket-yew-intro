use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub message: String,
    pub done: bool
}

impl Task {
    pub fn new(message: String) -> Self {
        Self {
            message, done: false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let task = Task::new("Todo".to_string());
        assert_eq!(task.message, "Todo".to_string());
    }
}

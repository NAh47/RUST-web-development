use crate::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents a question in the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub header: String,
    pub body: String,
    pub categories: Option<Vec<String>>,
}

impl Question {
    /// Creates a new question with the provided details.
    pub fn new(id: String, header: String, body: String, categories: Option<Vec<String>>) -> Self {
        Self { id, header, body, categories }
    }
}

/// Manages a collection of questions in memory.
#[derive(Clone)]
pub struct QuestionList {
    questions: HashMap<String, Question>,
}

impl QuestionList {
    /// Initializes a new collection of questions.
    pub fn new() -> Self {
        let mut list = Self { questions: HashMap::new() };
        list.load_initial_questions();
        list
    }

    /// Loads initial questions into the collection for demonstration purposes.
    fn load_initial_questions(&mut self) {
        let sample_questions = vec![
            Question::new("1".to_string(), "What is Rust?".to_string(), "A systems programming language that aims for memory safety and concurrency.".to_string(), Some(vec!["programming", "rust"])),
            Question::new("2".to_string(), "Why use Rust?".to_string(), "It provides memory safety guarantees without a garbage collector.".to_string(), Some(vec!["memory", "safety"])),
            Question::new("3".to_string(), "How does ownership work in Rust?".to_string(), "Ownership rules help manage memory automatically in a safe manner.".to_string(), Some(vec!["ownership", "rust"])),
        ];

        for question in sample_questions {
            self.questions.insert(question.id.clone(), question);
        }
    }

    /// Adds a question to the collection.
    pub fn add_question(&mut self, question: Question) {
        self.questions.insert(question.id.clone(), question);
    }

    /// Returns all stored questions.
    pub fn get_all_questions(&self) -> Vec<Question> {
        self.questions.values().cloned().collect()
    }

    /// Finds and returns a specific question by its id.
    pub fn find_question(&self, id: &str) -> Option<&Question> {
        self.questions.get(id)
    }

    /// Updates an existing question.
    pub fn update_question(&mut self, id: &str, new_question: Question) -> Result<(), String> {
        match self.questions.get_mut(id) {
            Some(question) => {
                *question = new_question;
                Ok(())
            },
            None => Err("Question not found".to_string())
        }
    }

    /// Removes a question by its id.
    pub fn remove_question(&mut self, id: &str) -> Option<Question> {
        self.questions.remove(id)
    }
}


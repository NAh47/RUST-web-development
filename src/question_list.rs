use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub header: String,
    pub body: String,
    pub categories: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: String, header: String, body: String, categories: Option<Vec<String>>) -> Self {
        Self { id, header, body, categories }
    }
}

#[derive(Clone)]
pub struct QuestionList {
    questions: HashMap<String, Question>,
}

impl QuestionList {
    pub fn new() -> Self {
        let mut list = Self { questions: HashMap::new() };
        list.load_initial_questions();
        list
    }

    fn load_initial_questions(&mut self) {
        let sample_questions = vec![
            Question::new("1".to_string(), "What is Rust?".to_string(), "A systems programming language that aims for memory safety and concurrency.".to_string(), Some(vec!["programming".to_string(), "rust".to_string()])),
            Question::new("2".to_string(), "Why use Rust?".to_string(), "It provides memory safety guarantees without a garbage collector.".to_string(), Some(vec!["memory".to_string(), "safety".to_string()])),
            Question::new("3".to_string(), "How does ownership work in Rust?".to_string(), "Ownership rules help manage memory automatically in a safe manner.".to_string(), Some(vec!["ownership".to_string(), "rust".to_string()])),
        ];

        for question in sample_questions {
            self.questions.insert(question.id.clone(), question);
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.insert(question.id.clone(), question);
    }

    pub fn get_all_questions(&self) -> Vec<Question> {
        self.questions.values().cloned().collect()
    }

    pub fn find_question(&self, id: &str) -> Option<&Question> {
        self.questions.get(id)
    }

    pub fn update_question(&mut self, id: &str, new_question: Question) -> Result<(), String> {
        match self.questions.get_mut(id) {
            Some(question) => {
                *question = new_question;
                Ok(())
            },
            None => Err("Question not found".to_string())
        }
    }

    pub fn remove_question(&mut self, id: &str) -> Option<Question> {
        self.questions.remove(id)
    }
}

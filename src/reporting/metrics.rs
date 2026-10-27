use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub results: Vec<TestResult>,
    pub started_at: String,
}

impl TestSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            results: Vec::new(),
            started_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn add_result(&mut self, name: impl Into<String>, passed: bool, message: impl Into<String>) {
        self.results.push(TestResult {
            name: name.into(),
            passed,
            message: message.into(),
            duration_ms: 0,
        });
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.len() - self.passed_count()
    }
}

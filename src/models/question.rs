use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub file_id: String,
    pub io: Vec<TestCase>,
}

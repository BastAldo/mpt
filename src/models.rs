use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WorkoutItem {
    Exercise(Exercise),
    Rest(Rest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exercise {
    pub name: String,
    pub duration: u32, // in seconds
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rest {
    pub duration: u32, // in seconds
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workout {
    pub date: String, // YYYY-MM-DD
    pub items: Vec<WorkoutItem>,
}

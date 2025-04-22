use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

// Displayトレイトの実装 - Priorityを人間が読みやすい形式で表示
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "低"),
            Priority::Medium => write!(f, "中"),
            Priority::High => write!(f, "高"),
        }
    }
}

struct Task {
    id: usize,
    title: String,
    completed: bool,
    priority: Priority,
    due_date: Option<DateTime<Local>>,
    created_at: DateTime<Local>,
}

fn main() {
    println!("Hello, world!");
}

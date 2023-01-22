use chrono::Duration;

#[derive(PartialEq, Clone)]
pub struct Item {
    pub text: String,
    pub duration: Duration
}

#[derive(PartialEq, Clone)]
pub struct Entry {
    pub items: Vec<Item>,
}
use chrono::Duration;

#[derive(PartialEq)]
pub struct Item {
    pub text: String,
    pub duration: Duration
}

#[derive(PartialEq)]
pub struct Entry {
    pub items: Vec<Item>,
}
use chrono::Duration;

#[derive(PartialEq, Clone)]
pub struct Item {
    pub text: String,
    pub duration: Duration
}

#[derive(PartialEq, Clone)]
pub struct Entry {
    pub date: chrono::NaiveDate,
    pub items: Vec<Item>,
}
#[derive(PartialEq)]
pub struct Item {
    pub text: String,
}

#[derive(PartialEq)]
pub struct Entry {
    pub items: Vec<Item>,
}
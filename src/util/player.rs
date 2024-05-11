pub struct Player {
    pub name: String,
    pub id: String,
    pub cards: Vec<i8>,
}

impl Player {
    pub fn new(name: String, id: String) -> Self {
        Player {
            name,
            id,
            cards: vec![],
        }
    }

    // Implement other Player methods as needed
}

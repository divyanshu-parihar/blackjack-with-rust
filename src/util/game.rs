use super::player::Player;

pub struct Game {
    pub cards: Vec<i8>,
    pub used_cards: Vec<i8>,
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player1_name: String, player2_name: String) -> Self {
        Game {
            cards: [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10,
                10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10,
                10, 10,
            ]
            .to_vec(),
            used_cards: vec![],
            player1: Player::new(player1_name, nanoid::nanoid!()),
            player2: Player::new(player2_name, nanoid::nanoid!()),
        }
    }

    // Implement other Game methods as needed
}

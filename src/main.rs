// // // Welcome to my game Blackjack.
// // // CLI version

// use nanoid::nanoid;
// use rand::prelude::*;
// use std::io::stdin;
// #[derive(Debug)]
// struct Player {
//     name: String,
//     id: String, //=> "Yo1Tr9F3iF-LFHX9i9GvA"
//     cards: Vec<i8>,
// }
// #[derive(Debug)]
// struct GameManager {
//     past_game: Vec<Game>,
//     current_game: Game,
// }

// #[derive(Debug)]
// struct Game {
//     cards: Vec<i8>,
//     used_cards: Vec<i8>,
//     // winner: Player,
//     pub player1: Player,
//     pub player2: Player,
// }
// impl GameManager {
//     fn new() -> Self {
//         GameManager {
//             current_game: Game {
//                 cards: [
//                     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10,
//                     10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9,
//                     10, 10, 10, 10,
//                 ]
//                 .to_vec(),
//                 used_cards: [].to_vec(),
//                 player1: Player {
//                     name: String::from("Divyanshu"),
//                     id: nanoid!(),
//                     cards: [].to_vec(),
//                 },
//                 player2: Player {
//                     name: String::from("X"),
//                     id: nanoid!(),
//                     cards: [].to_vec(),
//                 },
//             },
//             past_game: Vec::new(),
//         }
//     }
// }

// fn main() {
//     println!("Starting the Game!");
//     let mut gm = GameManager::new();

//     println!("Player one turn! ");
//     loop {
//         let mut input = String::new();
//         stdin()
//             .read_line(&mut input)
//             .expect("Please provide correct value");

//         let value: char = input.trim().parse().expect("Error happened!");

//         match value {
//             'q' => {
//                 println!("Player 1 left the game. Player 2 wins");
//                 break;
//             }
//             'h' => {
//                 let mut rng = rand::thread_rng();
//                 let random_number = rng.gen_range(0..=52);
//                 gm.current_game
//                     .player1
//                     .cards
//                     .push(gm.current_game.cards[random_number]);
//                 gm.current_game.cards.remove(random_number);

//                 let sum: i8 = gm.current_game.player1.cards.iter().sum();
//                 if sum > 21 {
//                     println!("Player 1 has burst! Player 2 win");
//                     break;
//                 }
//                 println!("Card Drawn : {}", gm.current_game.cards[random_number]);
//                 println!("current sum : {}", sum);
//             }
//             's' => {
//                 println!("Player 1 has choosen stand");
//                 println!("player 2 turn");
//                 loop {
//                     let mut input = String::new();
//                     stdin()
//                         .read_line(&mut input)
//                         .expect("Please provide correct value");

//                     let value: char = input.trim().parse().expect("Error happened!");

//                     match value {
//                         'q' => {
//                             println!("Player 1 left the game. Player 2 wins");
//                             break;
//                         }
//                         'h' => {
//                             let mut rng = rand::thread_rng();
//                             let random_number = rng.gen_range(0..=gm.current_game.cards.len() - 1);
//                             gm.current_game
//                                 .player2
//                                 .cards
//                                 .push(gm.current_game.cards[random_number]);
//                             gm.current_game.cards.remove(random_number);
//                             let sum: i8 = gm.current_game.player2.cards.iter().sum();
//                             if sum > 21 {
//                                 println!("Player 2 has burst! Player 1 win");
//                                 break;
//                             }
//                             println!("Card Drawn : {}", gm.current_game.cards[random_number]);
//                             println!("current sum : {}", sum);
//                         }
//                         's' => {
//                             println!("Player 2 has choosen to  stand");

//                             let sum1: i8 = gm.current_game.player1.cards.iter().sum();
//                             let sum2: i8 = gm.current_game.player2.cards.iter().sum();
//                             if sum1 == sum2 {
//                                 println!("It's a tie.");
//                             } else if sum1 > sum2 {
//                                 println!("Player 1 won.");
//                             } else {
//                                 println!("Player 2 won.")
//                             }
//                             break;
//                         }
//                         _ => {
//                             println!("Please only input q/h/s")
//                         }
//                     }
//                 }
//                 break;
//             }
//             _ => {
//                 println!("Please only input q/h/s")
//             }
//         }
//     }
// }

mod util;

use std::io::stdin;

use rand::prelude::*;
use util::game::Game;
fn main() {
    println!("Starting the Game!");
    let mut gm = Game::new("Divyanshu".to_string(), "X".to_string());

    println!("Player one turn! ");
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Please provide correct value");

        let value: char = input.trim().parse().expect("Error happened!");

        match value {
            'q' => {
                println!("Player 1 left the game. Player 2 wins");
                break;
            }
            'h' => {
                let mut rng = rand::thread_rng();
                let random_number = rng.gen_range(0..=52);
                gm.player1.cards.push(gm.cards[random_number]);
                gm.cards.remove(random_number);

                let sum: i8 = gm.player1.cards.iter().sum();
                if sum > 21 {
                    println!("Player 1 has burst! Player 2 win");
                    break;
                }
                println!("Card Drawn : {}", gm.cards[random_number]);
                println!("current sum : {}", sum);
            }
            's' => {
                println!("Player 1 has choosen stand");
                println!("player 2 turn");
                loop {
                    let mut input = String::new();
                    stdin()
                        .read_line(&mut input)
                        .expect("Please provide correct value");

                    let value: char = input.trim().parse().expect("Error happened!");

                    match value {
                        'q' => {
                            println!("Player 1 left the game. Player 2 wins");
                            break;
                        }
                        'h' => {
                            let mut rng = rand::thread_rng();
                            let random_number = rng.gen_range(0..=gm.cards.len() - 1);
                            gm.player2.cards.push(gm.cards[random_number]);
                            gm.cards.remove(random_number);
                            let sum: i8 = gm.player2.cards.iter().sum();
                            if sum > 21 {
                                println!("Player 2 has burst! Player 1 win");
                                break;
                            }
                            println!("Card Drawn : {}", gm.cards[random_number]);
                            println!("current sum : {}", sum);
                        }
                        's' => {
                            println!("Player 2 has choosen to  stand");

                            let sum1: i8 = gm.player1.cards.iter().sum();
                            let sum2: i8 = gm.player2.cards.iter().sum();
                            if sum1 == sum2 {
                                println!("It's a tie.");
                            } else if sum1 > sum2 {
                                println!("Player 1 won.");
                            } else {
                                println!("Player 2 won.")
                            }
                            break;
                        }
                        _ => {
                            println!("Please only input q/h/s")
                        }
                    }
                }
                break;
            }
            _ => {
                println!("Please only input q/h/s")
            }
        }
    }
}

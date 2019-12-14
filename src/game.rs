use crate::board::Board;
use std::io;

enum Turn {
    Player,
    Bot
}

pub struct Game {
    board: Board,
    turn: Turn
}

impl Game {
    pub fn new() -> Self {
        Self {
            turn: Turn::Player,
            board: Board::new()
        }
    }

    pub fn play(&mut self) {
        // let mut finished = false;
        // define a winner?
        self.play_turn();
    }

    fn play_turn(&mut self) {
        let moviment = self.player_move(&self.turn);
        self.valid_move(moviment);
        // if valid movement -> else await movement
        // change turn to other player
        self.print(false);
    }

    fn player_move(&self, turn: &Turn) -> Option {
        match turn {
            Turn::Player => {
                // print positions
                self.print(true);
                // request and return position marked
                println!("what position want to move?");
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(n) => input,
                    Err(error) => println!("error: {}", error)
                }
            }
            Turn::Bot => {
                &String::from("5")
                // return position
            }
        }
    }

    fn valid_move(&self, position: u32) {
        self.board.find_by_position(position)?.available
    }

	fn print(&self, position: bool) {
        let separator = "-------------";

        println!("\n{}", separator);

        for cell in self.board.cells.iter() {
            let content = if position {
                cell.position.to_string()
            } else {
                cell.mark
            };

            if cell.position % 3 == 0 {
                print!("| {} |\n{}\n", content, separator);
            } else {
                print!("| {} ", content);
            }
        };

        print!("\n");
    }
}

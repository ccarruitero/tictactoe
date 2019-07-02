use crate::board::Board;

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
        let mut finished = false;

        // while !finished {
            self.play_turn();
        // }
    }

    fn play_turn(&mut self) {
        self.print();
    }

	fn print(&self) {
        let separator = "-------------";

        println!("\n{}", separator);

        for cell in self.board.cells.iter() {
            if cell.position % 3 == 0 {
                print!("| {} |\n{}\n", cell.mark, separator);
            } else {
                print!("| {} ", cell.mark);
            }
        };

        print!("\n");
    }
}

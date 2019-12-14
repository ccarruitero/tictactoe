pub struct Board {
    pub cells: Vec<Cell>
}

pub struct Cell {
   	pub available: bool,
   	pub mark: String,
   	pub position: u8
}

impl Board {
    pub fn new() -> Self {
        let mut cells = vec![];
        for n in 1..=9 {
            cells.push(
                Cell{
                    available: true,
                    position: n,
                    mark: " ".to_string()
                }
            );
        };
        Self {
            cells: cells
        }
    }

    pub fn find_by_position(&self, n: u32) -> <Result(_, Err)> {
        let cell = self.cells.into_iter().filter(|&x| x.position == n)
    }
}

#[test]
fn test_board_new() {
    let board = Board::new();
    assert_eq!(board.cells.len(), 9);
}

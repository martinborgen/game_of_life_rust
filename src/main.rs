use std::io;

#[derive(PartialEq)]
struct Square {
    alive: bool,
    row: usize,
    col: usize,
    neighbours_alive: u32,
}

struct Board {
    board: Vec<Vec<Square>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn with_size(rows: usize, cols: usize) -> Board {
        let tmp = vec![vec![false; cols]; rows];
        let mut out = Board {
            board: Vec::new(),
            rows,
            cols,
        };
        Board::set_board(&mut out, tmp);
        out
    }

    fn from_vec(vec: Vec<Vec<bool>>) -> Board {
        let rows = vec.len();
        let cols = if rows > 0 { vec[0].len() } else { 0 };
        let mut out = Board {
            board: Vec::new(),
            rows,
            cols,
        };
        Board::set_board(&mut out, vec);
        out
    }

    fn get_neighbours(&self, m: usize, n: usize) -> Vec<&Square> {
        let mut output = Vec::new();

        let mut imin = 0;
        let mut jmin = 0;
        if m > 0 {
            imin = m - 1;
        }
        if n > 0 {
            jmin = n - 1;
        }

        for i in imin..std::cmp::min(m + 2, self.board.len()) {
            for j in jmin..std::cmp::min(n + 2, self.board[0].len()) {
                output.push(&self.board[i][j]);
            }
        }

        output
    }

    fn set_board(&mut self, vec: Vec<Vec<bool>>) {
        self.board.clear();
        for (i, i_vec) in vec.iter().enumerate() {
            self.board.push(Vec::new());
            for j in 0..i_vec.len() {
                self.board[i].push(Square {
                    alive: vec[i][j],
                    row: i,
                    col: j,
                    neighbours_alive: 0,
                });
            }
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, alive: bool) -> Result<(), &str> {
        if row > self.rows || col > self.cols {
            return Err("Index out of range");
        }
        self.board[row][col].alive = alive;
        Ok(())
    }

    fn print_board(&self) {
        for i in &self.board {
            for j in i {
                if j.alive {
                    print!("█");
                } else {
                    print!("▒");
                }
            }
            println!();
        }
    }

    fn advance_state(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let count = self.board[i][j].count_living_neighbours(self);
                self.board[i][j].neighbours_alive = count;
            }
        }

        for i in &mut self.board {
            for j in i {
                j.update_status();
            }
        }
    }
}

impl Square {
    fn count_living_neighbours(&self, board: &Board) -> u32 {
        let mut count = 0;
        let neighbours: Vec<&Square> = board.get_neighbours(self.row, self.col);
        for cell in neighbours {
            if cell == self {
                continue;
            } else if cell.alive {
                count += 1;
            }
        }
        count
    }

    fn update_status(&mut self) {
        if self.alive {
            if self.neighbours_alive < 2 || self.neighbours_alive > 3 {
                self.alive = false;
            }
        } else if self.neighbours_alive == 3 {
            self.alive = true;
        }
    }
}

fn main() {
    let toad = vec![
        vec![false, false, true, false],
        vec![true, false, false, true],
        vec![true, false, false, true],
        vec![false, true, false, false],
    ];

    let mut board = Board::from_vec(toad);

    board.print_board();

    let mut input = String::new();
    while input.trim().to_uppercase() != "Q" {
        input.clear();
        println!("iteration, enter to continue",);
        let _ = io::stdin().read_line(&mut input);
        board.advance_state();
        board.print_board();
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::Board;

    #[test]
    fn test_assign_board_from_vec() {
        let blinker_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let blinker = Board::from_vec(blinker_vec);

        assert_eq!(blinker.rows, 3);
        assert_eq!(blinker.cols, 3);
        assert!(!blinker.board.is_empty());

        assert_eq!(blinker.board[1][0].alive, false);
        assert_eq!(blinker.board[0][1].alive, true);
    }

    #[test]
    fn test_init_from_empty_vec() {
        let empty = Board::from_vec(vec![]);

        assert_eq!(empty.rows, 0);
        assert_eq!(empty.cols, 0);
        assert!(empty.board.is_empty());
    }

    #[test]
    fn test_init_empty_board() {
        let with_size = Board::with_size(8, 24);

        assert_eq!(with_size.rows, 8);
        assert_eq!(with_size.cols, 24);
        assert!(!with_size.board.is_empty());
        for row in with_size.board {
            for cell in row {
                assert!(
                    !cell.alive,
                    "Verifying cells are initialized as dead, cell ({},{}) wasn't",
                    cell.row, cell.col
                );
            }
        }
    }

    #[test]
    fn test_advance_state() {
        let blinker_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let mut blinker = Board::from_vec(blinker_vec);

        blinker.advance_state();

        assert_eq!(blinker.board[1][0].alive, true);
        assert_eq!(blinker.board[0][1].alive, false);

        blinker.advance_state();

        assert_eq!(blinker.board[1][0].alive, false);
        assert_eq!(blinker.board[0][1].alive, true);
    }

    #[test]
    fn set_cell_out_of_bounds() {
        let blinker_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let mut blinker = Board::from_vec(blinker_vec);

        let res = blinker.set_cell(10, 12, true);
        assert_eq!(res, Err("Index out of range"));
    }
}

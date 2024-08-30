#[derive(PartialEq)]
pub(crate) struct Square {
    pub(crate) alive: bool,
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) neighbours_alive: u32,
}

pub(crate) struct Board {
    pub(crate) board: Vec<Vec<Square>>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

#[allow(dead_code)]
impl Board {
    pub(crate) fn with_size(rows: usize, cols: usize) -> Board {
        let tmp = vec![vec![false; cols]; rows];
        let mut out = Board {
            board: Vec::new(),
            rows,
            cols,
        };
        Board::set_board(&mut out, tmp);
        out
    }

    pub(crate) fn from_vec(vec: Vec<Vec<bool>>) -> Board {
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

    pub(crate) fn get_neighbours(&self, m: usize, n: usize) -> Vec<&Square> {
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

    pub(crate) fn set_board(&mut self, vec: Vec<Vec<bool>>) {
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

    pub(crate) fn set_cell(&mut self, row: usize, col: usize, alive: bool) -> Result<(), &str> {
        if row > self.rows || col > self.cols {
            return Err("Index out of range");
        }
        self.board[row][col].alive = alive;
        Ok(())
    }

    pub(crate) fn print_board(&self) {
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

    pub(crate) fn advance_state(&mut self) {
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
    pub(crate) fn count_living_neighbours(&self, board: &Board) -> u32 {
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

    pub(crate) fn update_status(&mut self) {
        if self.alive {
            if self.neighbours_alive < 2 || self.neighbours_alive > 3 {
                self.alive = false;
            }
        } else if self.neighbours_alive == 3 {
            self.alive = true;
        }
    }
}

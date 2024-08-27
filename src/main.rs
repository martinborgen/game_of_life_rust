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
    fn from_vec(vec: Vec<Vec<bool>>) -> Board {
        let rows = vec.len();
        let cols;
        if rows > 0 {
            cols = vec[0].len();
        } else {
            cols = 0;
        }
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

        return output;
    }

    fn set_board(&mut self, vec: Vec<Vec<bool>>) {
        for i in 0..vec.len() {
            self.board.push(Vec::new());
            for j in 0..vec[i].len() {
                self.board[i].push(Square {
                    alive: vec[i][j],
                    row: i,
                    col: j,
                    neighbours_alive: 0,
                });
            }
        }
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
        } else {
            if self.neighbours_alive == 3 {
                self.alive = true;
            }
        }
    }
}

fn main() {
    let blinker = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];

    let toad = vec![
        vec![false, false, true, false],
        vec![true, false, false, true],
        vec![true, false, false, true],
        vec![false, true, false, false],
    ];

    let tmp = Board::from_vec(vec![]);
    let mut board = Board::from_vec(toad);

    board.print_board();

    let mut input = String::new();
    while input.trim() != String::from("Q") {
        input.clear();
        println!("iteration, enter to continue",);
        let _ = io::stdin().read_line(&mut input);
        board.advance_state();
        board.print_board();
    }
}

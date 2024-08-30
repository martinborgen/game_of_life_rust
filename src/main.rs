use std::io;

mod game;

fn main() {
    let toad = vec![
        vec![false, false, true, false],
        vec![true, false, false, true],
        vec![true, false, false, true],
        vec![false, true, false, false],
    ];

    let mut board = game::Board::from_vec(toad);

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
    use super::*;

    #[test]
    fn test_assign_board_from_vec() {
        let blinker_vec = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let blinker = game::Board::from_vec(blinker_vec);

        assert_eq!(blinker.rows, 3);
        assert_eq!(blinker.cols, 3);
        assert!(!blinker.board.is_empty());

        assert_eq!(blinker.board[1][0].alive, false);
        assert_eq!(blinker.board[0][1].alive, true);
    }

    #[test]
    fn test_init_from_empty_vec() {
        let empty = game::Board::from_vec(vec![]);

        assert_eq!(empty.rows, 0);
        assert_eq!(empty.cols, 0);
        assert!(empty.board.is_empty());
    }

    #[test]
    fn test_init_empty_board() {
        let with_size = game::Board::with_size(8, 24);

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
        let mut blinker = game::Board::from_vec(blinker_vec);

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
        let mut blinker = game::Board::from_vec(blinker_vec);

        let res = blinker.set_cell(10, 12, true);
        assert_eq!(res, Err("Index out of range"));
    }
}

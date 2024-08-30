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

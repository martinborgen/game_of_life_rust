use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

mod game;

fn make_board() -> game::Board {
    let toad = vec![
        vec![false, false, true, false],
        vec![true, false, false, true],
        vec![true, false, false, true],
        vec![false, true, false, false],
    ];

    let board = game::Board::from_vec(toad);

    board
}

fn run(mut terminal: DefaultTerminal, board: &mut game::Board) -> io::Result<()> {
    loop {
        // terminal.draw(|frame| {
        //     let greeting =
        //         Paragraph::new("Conway's Game of Life, by Martin Borgén\nPress q to quit")
        //             .white()
        //             .on_blue();
        //     frame.render_widget(greeting, frame.area());
        // })?;

        terminal.draw(|frame| {
            let game = Paragraph::new(board.to_string()).white().on_blue();
            frame.render_widget(game, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
            {
                return Ok(());
            }
            if key.kind == KeyEventKind::Press
                && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' '))
            {
                board.advance_state();
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut board = make_board();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, &mut board);
    ratatui::restore();
    app_result
}

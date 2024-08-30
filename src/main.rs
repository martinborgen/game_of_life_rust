use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

mod game;

mod tui;

#[derive(Debug, Default)]
struct GameApp {
    board: game::Board,
    exit: bool,
}

impl GameApp {
    pub fn run(&mut self, terminal: &mut tui::Tui) {
        while !self.exit {
            let _ = terminal.draw(|frame| self.render_frame(frame));

            if let Ok(event::Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    self.exit();
                }

                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' '))
                {
                    self.board.advance_state();
                }
            } else {
                self.exit();
            }
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

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

impl Widget for &GameApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from("Game Of Life".bold());
        let instructions = Title::from(Line::from(vec![
            "Advance state: ".into(),
            "<spacebar>".blue().bold(),
            " or ".into(),
            "<return>".blue().bold(),
            " Quit: ".into(),
            "<q>".blue().bold(),
        ]));
        let block = Block::new()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::PLAIN);

        let game_state = Text::from(self.board.to_string());

        Paragraph::new(game_state)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() {
    let mut game = GameApp {
        board: make_board(),
        exit: false,
    };
    let mut terminal = tui::init().unwrap();
    let _app_result = game.run(&mut terminal);
    ratatui::restore();
}

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Position, Rect},
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
    cursor: (u16, u16),
    editing: bool,
}

impl GameApp {
    pub fn run(&mut self, terminal: &mut tui::Tui) {
        while !self.exit {
            let _ = terminal.draw(|frame| self.render_frame(frame));

            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => self.cursor.0 -= 1,
                        KeyCode::Down => self.cursor.0 += 1,
                        KeyCode::Left => self.cursor.1 -= 1,
                        KeyCode::Right => self.cursor.1 += 1,
                        KeyCode::Char('e') => {
                            if !self.editing {
                                self.cursor = (0, 0);
                            }
                            self.editing = !self.editing;
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => self.board.advance_state(),
                        KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if self.editing {
            frame.set_cursor_position(Position {
                x: self.cursor.1,
                y: self.cursor.0,
            });
        }
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Game Of Life".bold());
        let instructions = Title::from(Line::from(vec![
            "Advance state: ".into(),
            "<spacebar>".blue().bold(),
            " or ".into(),
            "<return>".blue().bold(),
            " Quit: ".into(),
            "<q>".blue().bold(),
            " Edit cells: ".into(),
            "<e>".blue().bold(),
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
        cursor: (0, 0),
        editing: false,
    };
    let mut terminal = tui::init().unwrap();
    let _app_result = game.run(&mut terminal);
    ratatui::restore();
}

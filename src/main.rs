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
                        KeyCode::Up => {
                            if self.cursor.0 > 0 {
                                self.cursor.0 -= 1;
                            }
                        }
                        KeyCode::Down => self.cursor.0 += 1,
                        KeyCode::Left => {
                            if self.cursor.1 > 0 {
                                self.cursor.1 -= 1;
                            }
                        }
                        KeyCode::Right => self.cursor.1 += 1,
                        KeyCode::Char('e') => {
                            // if !self.editing {
                            //     self.cursor = (1, 0);
                            // }
                            self.editing = !self.editing;
                        }
                        KeyCode::Char(' ') => self.board.advance_state(),
                        KeyCode::Enter => {
                            if self.editing {
                                let cell = &mut self.board.board[self.cursor.0 as usize - 1]
                                    [self.cursor.1 as usize];
                                cell.alive = !cell.alive;
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let size = frame.area();
        self.board
            .resize_board(size.height as usize, size.width as usize);

        // The game uses x for rows, y for columns, while the UI uses x for columns and y for rows!
        if self.cursor.0 > size.height {
            self.cursor.0 = size.height;
        }
        if self.cursor.1 > size.width {
            self.cursor.1 = size.width;
        }

        frame.render_widget(&*self, frame.area());
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
            " Toggle cell alive (in editing mode): ".into(),
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
        cursor: (1, 0),
        editing: false,
    };
    let mut terminal = tui::init().unwrap();
    let _app_result = game.run(&mut terminal);
    ratatui::restore();
}

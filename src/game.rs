use crate::tetromino::*;
use std::io::{self, Write};
use termion::{
    color::{self, Bg, Fg},
    cursor,
    raw::*,
};

const HEIGHT: usize = 20;
const WIDTH: usize = 10;

pub struct Game {
    grid: [[Color; WIDTH]; HEIGHT],
    score: u32,
    tetromino: Tetromino,
    pub over: bool,
}

pub struct GameControls {
    pub game: Game,
    pub pause: bool,
    out: RawTerminal<io::Stdout>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: [[Color::None; WIDTH]; HEIGHT],
            score: 0,
            over: false,
            tetromino: Tetromino::new_random(WIDTH),
        }
    }

    fn draw_piece(&mut self, draw: bool) {
        for cell in &self.tetromino.cells {
            self.grid[cell.0][cell.1] = if draw {
                self.tetromino.color
            } else {
                Color::None
            }
        }
    }

    fn clear_lines(&mut self) {
        for i in 0..HEIGHT {
            let full = self.grid[i].iter().all(|x| x.is_some());
            if !full {
                continue;
            }
            self.score += 1;
            for k in (1..=i).rev() {
                let prev_row = self.grid[k - 1];
                self.grid[k] = prev_row;
            }
            self.grid[0] = [Color::None; WIDTH];
        }
    }

    pub fn shift(&mut self, dir: Direction) {
        let (left, right, down) = self.piece_touches();
        let touch = match dir {
            Direction::Left => left,
            Direction::Right => right,
            Direction::Down => down,
        };
        if !touch {
            self.tetromino.shift(dir);
        }
    }

    fn piece_touches(&self) -> (bool, bool, bool) {
        let cells = self.tetromino.cells;
        let left = cells
            .iter()
            .any(|cell| cell.1 == 0 || self.grid[cell.0][cell.1 - 1].is_some());
        let right = cells
            .iter()
            .any(|cell| cell.1 == WIDTH - 1 || self.grid[cell.0][cell.1 + 1].is_some());
        let down = cells
            .iter()
            .any(|cell| cell.0 == HEIGHT - 1 || self.grid[cell.0 + 1][cell.1].is_some());
        (left, right, down)
    }

    pub fn turn(&mut self) {
        let backup = self.tetromino.cells;
        if self.tetromino.turn().is_some() {
            let in_bounds = self.tetromino.cells.iter().all(|cell| {
                cell.0 < HEIGHT && cell.1 < WIDTH && self.grid[cell.0][cell.1].is_none()
            });
            if in_bounds {
                return;
            }
        }
        self.tetromino.cells = backup;
    }

    pub fn hard_drop(&mut self) {
        while !self.piece_touches().2 {
            self.shift(Direction::Down);
        }
    }

    pub fn tick(&mut self) {
        if !self.piece_touches().2 {
            self.shift(Direction::Down);
        } else {
            self.draw_piece(true);
            self.clear_lines();
            self.tetromino = Tetromino::new_random(WIDTH);
            self.over = self
                .tetromino
                .cells
                .iter()
                .any(|cell| self.grid[cell.0][cell.1].is_some());
        }
    }
}

impl GameControls {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}{}", cursor::Hide, termion::clear::All)?;
        Ok(GameControls {
            game: Game::new(),
            pause: false,
            out: stdout,
        })
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.game.draw_piece(true);
        write!(self.out, "{}", cursor::Goto(1, 1))?;
        let wall = format!("{} {}", Bg(color::White), Bg(color::Reset));
        for i in 0..HEIGHT {
            let row = (0..WIDTH)
                .map(|j| format!("{}  {}", self.game.grid[i][j], Bg(color::Reset)))
                .collect::<String>();
            writeln!(self.out, "{}{}{}\r", wall, row, wall)?;
        }
        let bottom = (0..=WIDTH).map(|_| "  ").collect::<String>();
        write!(
            self.out,
            "{}{}{}",
            Bg(color::White),
            Fg(color::Black),
            bottom
        )?;
        writeln!(
            self.out,
            "{} Score: {}{}{}\r",
            cursor::Goto(1, 1 + HEIGHT as u16),
            self.game.score,
            Bg(color::Reset),
            Fg(color::Reset)
        )?;
        self.game.draw_piece(false);
        self.out.flush()
    }

    pub fn shift(&mut self, dir: Direction) {
        if !self.pause {
            self.game.shift(dir);
        }
    }

    pub fn turn(&mut self) {
        if !self.pause {
            self.game.turn();
        }
    }

    pub fn hard_drop(&mut self) {
        if !self.pause {
            self.game.hard_drop();
        }
    }

    pub fn tick(&mut self) {
        if !self.pause {
            self.game.tick();
        }
    }
}

impl Drop for GameControls {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        write!(self.out, "{}", cursor::Show);
        self.out.flush();
    }
}

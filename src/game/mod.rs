use crate::{game::board::*, ui::{Background, Asset}};
use curio_bsp::Button;
use klaptik::*;

pub mod board;

#[derive(Copy, Clone)]
pub enum GameStatus {
    Win,
    Bootstrap,
    Playing,
    GameOver,
}

pub struct Minesweeper {
    board: Board,
    status: GameStatus,
    bombs: usize,
    rng_seed: u32,
}

impl Minesweeper {
    pub fn new(bombs: usize) -> Self {
        Self {
            bombs,
            board: Board::new(),
            status: GameStatus::Bootstrap,
            rng_seed: 42,
        }
    }

    pub fn seed_random(&mut self, seed: u32) {
        self.rng_seed = seed % 0x7fff_ffff;
    }

    pub fn button_click(&mut self, button: Button) {
        let cursor = self.board.cursor();
        match button {
            Button::A => match self.status {
                GameStatus::Bootstrap => {
                    self.bootstrap();
                    self.open_tile(cursor)
                }
                GameStatus::Playing => {
                    self.open_tile(cursor);
                    self.refresh_game_state()
                }
                _ => self.bootstrap(),
            },
            Button::B => {
                match self.board.tile_at(cursor).status() {
                    TileStatus::Closed => self.board.set_status_at(cursor, TileStatus::Flagged),
                    TileStatus::Flagged => self.board.set_status_at(cursor, TileStatus::Closed),
                    _ => {}
                };
                self.refresh_game_state()
            }
            Button::Up if cursor.y > 0 => {
                self.board.move_cursor(Point::new(cursor.x, cursor.y - 1))
            }
            Button::Right if cursor.x + 1 < Board::WIDTH as u8 => {
                self.board.move_cursor(Point::new(cursor.x + 1, cursor.y))
            }
            Button::Down if cursor.y + 1 < Board::HEIGHT as u8 => {
                self.board.move_cursor(Point::new(cursor.x, cursor.y + 1))
            }
            Button::Left if cursor.x > 0 => {
                self.board.move_cursor(Point::new(cursor.x - 1, cursor.y))
            }
            _ => {}
        };
    }

    fn refresh_game_state(&mut self) {
        if self
            .board
            .tiles()
            .iter()
            .any(|&tile| tile.status() == TileStatus::Opened && tile.content() == TileContent::Bomb)
        {
            self.status = GameStatus::GameOver;
            return;
        }

        let win = self.board.tiles().iter().all(|&tile| {
            matches!(
                (tile.status(), tile.content()),
                (TileStatus::Flagged, TileContent::Bomb) | (TileStatus::Opened, _)
            )
        });

        if win {
            self.status = GameStatus::Win;
        }
    }

    fn open_tile(&mut self, origin: Point) {
        if let TileStatus::Closed = self.board.tile_at(origin).status() {
            match self.board.tile_at(origin).content() {
                TileContent::Hint(0) => {
                    self.board.set_status_at(origin, TileStatus::Opened);
                    for neighbor in Neighbors::at(origin) {
                        self.open_tile(neighbor);
                    }
                }
                _ => self.board.set_status_at(origin, TileStatus::Opened),
            }
        }
    }

    fn bootstrap(&mut self) {
        self.board.reset();

        let mut bombs_planted = 0;
        while bombs_planted < self.bombs {
            let pos = Point::new(
                self.gen_random(Board::WIDTH as u16),
                self.gen_random(Board::HEIGHT as u16),
            );
            match self.board.tile_at(pos).content() {
                TileContent::Hint(_) if pos != self.board.cursor() => {
                    self.board.set_content_at(pos, TileContent::Bomb);
                    bombs_planted += 1;
                }
                _ => {}
            }
        }

        for x in 0..Board::WIDTH {
            for y in 0..Board::HEIGHT {
                let pos = Point::new(x as u8, y as u8);

                if let TileContent::Bomb = self.board.tile_at(pos).content() {
                    continue;
                }

                let mut bombs = 0;
                for neighbor in Neighbors::at(pos) {
                    if let TileContent::Bomb = self.board.tile_at(neighbor).content() {
                        bombs += 1;
                    }
                }

                self.board.set_content_at(pos, TileContent::Hint(bombs));
            }
        }

        self.status = GameStatus::Playing;
    }

    fn gen_random(&mut self, up_to: u16) -> u8 {
        self.rng_seed = self.rng_seed * 16_807 % 0x7fff_ffff;
        (self.rng_seed % up_to as u32) as u8
    }
}

widget_group! {
    GameUI<&Minesweeper>,
    {
        bg: Background;
        logo: GlyphIcon, Asset::GameLogo, 0, Point::new(0, 0);
        game_screen: GameScreen;
    },
    |game_ui: &mut GameUI, state: &Minesweeper| {
        game_ui.game_screen.update(state);
    }
}

pub type GameWidget = WrapPanel<{ Board::TILES }, { Board::WIDTH as _ }>;

widget_mux!(
    GameScreen<&Minesweeper>,
    GameScreenNode::Board,
    {
        board: GameWidget, Asset::GameBoard, "", Point::new(0, 16), Size::new(8, 8);
        win: GlyphIcon, Asset::GamePopup, b'W', Point::new(24, 24);
        game_over: GlyphIcon, Asset::GamePopup, b'L', Point::new(24, 24);
    },
    |mux: &mut GameScreen, state: &Minesweeper| {
        let node = match state.status {
            GameStatus::GameOver => GameScreenNode::GameOver,
            GameStatus::Win => GameScreenNode::Win,
            _ => GameScreenNode::Board,
        };
        mux.set_active(node);
        let cursor_idx = state.board.cursor_offset();
        for (idx, tile) in state.board.tiles().iter().enumerate() {
            let mut glyph = tile.into();
            if idx == cursor_idx {
                glyph += 13;
            }
            mux.board.set_glyph(idx, glyph);
        }
    }
);

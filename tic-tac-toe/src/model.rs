use nannou::prelude::*;
use nannou_egui::Egui;
use std::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub enum CellState {
    Empty = 0,
    Circle = 1,
    Cross = 2,
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    NotStarted = 0,
    InGame = 1,
    GameOver = 2,
    Tie = 3,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    None = 0,
    PCircle = 1,
    PCross = 2,
    Tie = 3,
}

pub struct Model {
    pub state: Vec<CellState>,
    pub player: CellState,
    pub egui: Egui,
    pub cricle_score: u16,
    pub cross_score: u16,
    pub game_state: GameState,
    pub winning_combination: i8,
    pub should_clear_state: bool,
}

impl Model {
    pub fn new(window: &Window) -> Self {
        let mut game_state: Vec<CellState> = Vec::new();
        for _ in 0..9 {
            game_state.push(CellState::Empty);
        }

        Model {
            state: game_state,
            player: CellState::Circle,
            egui: Egui::from_window(window),
            cricle_score: 0,
            cross_score: 0,
            game_state: GameState::NotStarted,
            winning_combination: -1,
            should_clear_state: false,
        }
    }
}

pub const WINNER_COMBINATIONS: [[u8; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

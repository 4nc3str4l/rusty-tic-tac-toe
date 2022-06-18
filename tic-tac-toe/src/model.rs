use std::vec::Vec;
use nannou::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum CellState {
    EMPTY = 0,
    CIRCLE = 1,
    CROSS = 2
}

pub struct Model {
    pub state: Vec<CellState>,
    pub player: CellState
}

impl Model {
    pub fn new() -> Self {
        let mut game_state: Vec<CellState> = Vec::new();
        for _ in 0..9 {
            game_state.push(CellState::EMPTY);
        }
        return Model { state: game_state, player: CellState::CIRCLE }
    }
}


pub fn model(_app: &App) -> Model {
    Model::new()
}
use std::vec::Vec;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};


#[derive(PartialEq, Clone, Copy)]
pub enum CellState {
    EMPTY = 0,
    CIRCLE = 1,
    CROSS = 2
}

pub struct Settings {
    pub resolution: u32,
    pub scale: f32,
    pub rotation: f32,
    pub color: Srgb<u8>,
    pub position: Vec2,
}

pub struct Model {
    pub state: Vec<CellState>,
    pub player: CellState,
    pub egui: Egui,
    pub settings: Settings,
}

impl Model {
    pub fn new(window: &Window) -> Self {
        let mut game_state: Vec<CellState> = Vec::new();
        for _ in 0..9 {
            game_state.push(CellState::EMPTY);
        }
        return Model { state: game_state,
                       player: CellState::CIRCLE,
                       egui: Egui::from_window(window),
                       settings: Settings {
                            resolution: 10,
                            scale: 200.0,
                            rotation: 0.0,
                            color: WHITE,
                            position: vec2(0.0, 0.0),
                        },
                    }
    }
}



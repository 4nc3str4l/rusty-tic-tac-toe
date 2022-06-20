use nannou::prelude::*;
use nannou_egui::egui;

mod model;
mod rendering;
mod utils;

use crate::{model::*, rendering::*, utils::*};

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    if model.game_state == GameState::InGame {
        update_ingame(app, model);
    }
    render_gui(model, &update);

    // Clear the board if needed
    if model.should_clear_state {
        clear_board(model);
        model.should_clear_state = false;
    }
}

fn update_ingame(app: &App, model: &mut Model) {
    if app.mouse.buttons.left().is_down() {
        let (x, y) = mouse_to_cell(&app);

        let cell_state = (*(*model).state)[(y * 3 + x) as usize];
        if cell_state != CellState::Empty {
            return;
        }

        match (*model).player {
            CellState::Circle => {
                set_cell_state(model, x, y, CellState::Circle);
                (*model).player = CellState::Cross;
            }
            CellState::Cross => {
                set_cell_state(model, x, y, CellState::Cross);
                (*model).player = CellState::Circle;
            }
            CellState::Empty => {}
        }

        let (winner, combiantion) = check_winner(model);
        if winner != Player::None {
            model.game_state = GameState::GameOver;
            model.winning_combination = combiantion;
            if winner == Player::PCircle {
                model.p1_score += 1;
            } else {
                model.p2_score += 1;
            }
        }
    }
}

fn set_cell_state(model: &mut Model, x: i32, y: i32, state: CellState) {
    (*(*model).state)[(y * 3 + x) as usize] = state
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    if model.game_state != GameState::NotStarted {
        let (x, y) = mouse_to_cell(&app);

        if model.game_state == GameState::InGame {
            highlight_cell(&draw, app, x, y, false);
        }
        draw_grid(&draw, &app);

        // Draw elements
        for i in 0..3 {
            for j in 0..3 {
                let el = model.state.get(ij_to_idx(i, j)).unwrap();
                draw_element(&draw, app, j as i32, i as i32, &el);
            }
        }

        if model.game_state == GameState::GameOver {
            for c_idx in WINNER_COMBINATIONS[model.winning_combination as usize] {
                let (x, y) = idx_to_ij(c_idx as usize);
                highlight_cell(&draw, app, x as i32, y as i32, true);
            }
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

pub fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    window.set_title("Rusty Tic-Tac-Toe");
    Model::new(&window)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn render_gui(model: &mut Model, update: &Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    if model.game_state == GameState::InGame {
        return;
    }

    egui::Window::new("Game Menu").show(&ctx, |ui| {
        if model.game_state == GameState::NotStarted {
            let clicked = ui.button("New Game").clicked();
            if clicked {
                model.game_state = GameState::InGame;
            }
        } else if model.game_state == GameState::GameOver {
            ui.label("Score:");
            ui.label("Player 1:");
            ui.label(model.p1_score);
            ui.label("Player 2:");
            ui.label(model.p2_score);
            let clicked = ui.button("Play Again").clicked();
            if clicked {
                model.game_state = GameState::InGame;
                model.should_clear_state = true;
            }
        }
    });
}

fn check_winner(model: &Model) -> (Player, i8) {
    // Check if the circle player won
    let p1_win = check_game_for_player(model, CellState::Circle);
    if p1_win != -1 {
        return (Player::PCircle, p1_win);
    }

    // Check if the cross player won
    let p2_win = check_game_for_player(model, CellState::Cross);
    if p2_win != -1 {
        return (Player::PCross, p2_win);
    }

    // If none of them won we can return -1
    (Player::None, -1)
}

fn check_game_for_player(model: &Model, target_state: CellState) -> i8 {
    for (i, combination) in WINNER_COMBINATIONS.iter().enumerate() {
        let mut won: bool = true;
        for c_idx in combination {
            if model.state[*c_idx as usize] != target_state {
                won = false;
                break;
            }
        }
        if won {
            return i as i8;
        }
    }
    -1
}

fn clear_board(model: &mut Model) {
    for i in 0..3 {
        for j in 0..3 {
            set_cell_state(model, j, i, CellState::Empty);
        }
    }
}

use nannou::prelude::*;

mod model;
mod rendering;
mod utils;

use crate::{model::*, rendering::*, utils::*};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn update(app: &App, model: &mut Model, update: Update) {

    if app.keys.down.contains(&Key::Space) {
        for i in 0..3 {
            for j in 0..3 {
                set_cell_state(model, j, i, CellState::EMPTY);
            }
        }
    }

    if app.mouse.buttons.left().is_down() {
        let (x, y) = mouse_to_cell(&app);

        let cell_state = (*(*model).state)[(y * 3 + x) as usize];
        if cell_state != CellState::EMPTY{
            return;
        }

        match (*model).player {
            CellState::CIRCLE => {
                set_cell_state(model, x, y, CellState::CIRCLE);
                (*model).player = CellState::CROSS;
            },
            CellState::CROSS => {
                set_cell_state(model, x, y, CellState::CROSS);
                (*model).player = CellState::CIRCLE;
            },
            CellState::EMPTY => {},
        }
    }
}

fn set_cell_state(model: &mut Model, x: i32, y: i32, state: CellState) {
    (*(*model).state)[(y * 3 + x) as usize] = state
}

fn view(app: &App,  model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    let (x, y) = mouse_to_cell(&app);
    draw_cell(&draw, app, x, y);
    draw_grid(&draw, &app);

    // Draw elements
    for i in 0..3{
        for j in 0..3 {
            let el = model.state.get(i*3 + j).unwrap();
            draw_element(&draw, app, j as i32, i as i32, &el);   
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}


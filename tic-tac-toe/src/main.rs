use nannou::prelude::*;
use nannou_egui::{egui, Egui};

mod model;
mod rendering;
mod utils;

use crate::{model::*, rendering::*, utils::*};

fn main() {
    nannou::app(model)
        .update(update)
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

    // Update the UI
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Resolution:");
        ui.add(egui::Slider::new(&mut settings.resolution, 1..=40));

        // Scale slider
        ui.label("Scale:");
        ui.add(egui::Slider::new(&mut settings.scale, 0.0..=1000.0));

        // Rotation slider
        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));

        // Random color button
        let clicked = ui.button("Random color").clicked();

        if clicked {
            settings.color = rgb(random(), random(), random());
        }
    });



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
    Model::new(&window)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
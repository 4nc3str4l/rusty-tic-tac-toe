use nannou::prelude::*;

use crate::utils::*;
use crate::model::*;

pub fn draw_grid(draw: &Draw, app: &App){
    let window_dimensions = app.main_window().inner_size_pixels();

    let x_third : f32 = (window_dimensions.0 as f32) / 3.0;
    let y_third : f32 = (window_dimensions.1 as f32) / 3.0;
    let (x_0, y_0) = win_start(&app);

    // vertical lines
    let start_point = pt2(x_0 + x_third, y_0);
    let end_point   = pt2(x_0 + x_third, -y_0);
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(WHITE);
    
    
    let start_point = pt2(x_0 + x_third * 2.0, y_0);
    let end_point   = pt2(x_0 + x_third * 2.0, -y_0);
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(WHITE);
    
    // Horizontal lines
    let start_point = pt2(-x_0, y_0 + y_third);
    let end_point   = pt2(x_0, y_0 + y_third);
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(WHITE);

    let start_point = pt2(-x_0, y_0 + y_third * 2.0);
    let end_point   = pt2(x_0, y_0 + y_third * 2.0);
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(WHITE);
}

pub fn draw_cell(draw: &Draw, app: &App, x: i32, y: i32) {
    let win_d = app.main_window().inner_size_pixels();
    let x_third : f32 = (win_d.0 as f32) / 3.;
    let y_third : f32 = (win_d.1 as f32) / 3.;
    let (x_0, y_0) = win_start(&app);
    
    draw.rect()
        .x(x_0 + (x as f32) * x_third + (x_third / 2.))
        .y(y_0 + (y as f32) * y_third + (y_third / 2.))
        .w(x_third)
        .h(y_third)
        .rgba(1.0, 1.0, 1.0, 0.1);
}

pub fn draw_element(draw: &Draw, app: &App, x: i32, y: i32, cell_state: &CellState) {
    if *cell_state == CellState::EMPTY {
        return;
    }

    let win_d = app.main_window().inner_size_pixels();
    let x_third : f32 = (win_d.0 as f32) / 3.;
    let y_third : f32 = (win_d.1 as f32) / 3.;
    let (x_0, y_0) = win_start(&app);

    let dim = f32::min(x_third, y_third);

    let center_x = x_0 + (x as f32) * x_third + (x_third / 2.);
    let center_y = y_0 + (y as f32) * y_third + (y_third / 2.);

    if *cell_state == CellState::CIRCLE {
        draw.ellipse()
            .x(center_x)
            .y(center_y)
            .w(dim * 0.8)
            .h(dim * 0.8)
            .color(WHITE);
        
        draw.ellipse()
            .x(center_x)
            .y(center_y)
            .w(dim * 0.6)
            .h(dim * 0.6).color(BLACK);
    } else {
        let x_length = dim * 0.7;
        let p0_1 = pt2(center_x - x_length / 2., center_y - x_length / 2.);
        let p1_1   = pt2(center_x + x_length / 2., center_y + x_length / 2.);

        draw.line()
        .start(p0_1)
        .end(p1_1)
        .weight(dim*0.1)
        .color(WHITE);
        
        let p0_2 = pt2(center_x - x_length / 2., center_y + x_length / 2.);
        let p1_2   = pt2(center_x + x_length / 2., center_y - x_length / 2.);
        draw.line()
            .start(p0_2)
            .end(p1_2)
            .weight(dim*0.1)
            .color(WHITE);
    }
}
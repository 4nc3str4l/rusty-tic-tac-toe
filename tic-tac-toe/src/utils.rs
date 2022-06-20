use nannou::prelude::*;

pub fn mouse_to_cell(app: &App) -> (i32, i32) {
    let m_pos = app.mouse.position();
    let (w_start_x, w_start_y) = win_start(&app);
    let (w_dim_x, w_dim_y) = win_dim(&app);
    for y in 0..3 {
        for x in 0..3 {
            let x0 = w_start_x + (w_dim_x / 3. * x as f32);
            let x1 = w_start_x + (w_dim_x / 3. * (x + 1) as f32);

            let y0 = w_start_y + (w_dim_y / 3. * y as f32);
            let y1 = w_start_y + (w_dim_y / 3. * (y + 1) as f32);

            if in_range(m_pos.x, x0, x1) && in_range(m_pos.y, y0, y1) {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

pub fn in_range(x: f32, x0: f32, x1: f32) -> bool {
    x >= x0 && x < x1
}

pub fn win_start(app: &App) -> (f32, f32) {
    let win_d = app.main_window().inner_size_pixels();
    (-(win_d.0 as f32) / 2., -(win_d.1 as f32) / 2.)
}

pub fn win_dim(app: &App) -> (f32, f32) {
    let win_d = app.main_window().inner_size_pixels();
    (win_d.0 as f32, win_d.1 as f32)
}

pub fn ij_to_idx(i: usize, j: usize) -> usize {
    i * 3 + j
}

pub fn idx_to_ij(idx: usize) -> (usize, usize) {
    (idx % 3, idx / 3)
}

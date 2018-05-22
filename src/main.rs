extern crate cgmath;
extern crate piston_window;
extern crate rayon;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

mod shapes;
mod solvers;
mod types;

use types::*;

use shapes::*;

fn make_cells() -> Cells {
    let mut v = Vec::with_capacity(CELLS_HIGH * CELLS_WIDE);
    for _ in 0..(CELLS_HIGH * CELLS_WIDE) {
        v.push(Mutex::new([0.1f32, 0.1f32, 0.1f32, 1.0f32]));
    }
    Arc::new(v)
}

fn trace() -> Color {
    [1.0, 0.1, 0.8, 1.0]
}

fn get_xy(index: usize) -> (usize, usize) {
    (index / CELLS_WIDE, index % CELLS_WIDE)
}

fn get_index(x: usize, y: usize) -> usize {
    x * CELLS_WIDE + y
}

fn trace_rays(cells: Cells) {
    use rayon::prelude::*;

    cells.par_iter().for_each(|cell| {
        std::thread::sleep(std::time::Duration::new(0, 5_000_000));

        let color = trace();

        'try_update: loop {
            match cell.lock() {
                Ok(mut c) => {
                    c[0] = color[0];
                    c[1] = color[1];
                    c[2] = color[2];
                    c[3] = color[3];
                    break 'try_update;
                }
                Err(_) => println!("Hit lock."),
            }
        }
    });
}

fn render_screen<G>(cells: &Cells, screen_size: Size, transform: [[f64; 3]; 2], g: &mut G)
where
    G: Graphics,
{
    for x in 0..CELLS_WIDE {
        for y in 0..CELLS_HIGH {
            let Size { width, height } = screen_size;
            let cell_width = width as f64 / CELLS_WIDE as f64;
            let cell_height = height as f64 / CELLS_HIGH as f64;

            match cells[get_index(x, y)].lock() {
                Ok(cell) => {
                    rectangle(
                        [cell[0], cell[1], cell[2], cell[3]],
                        [
                            x as f64 * cell_width,
                            y as f64 * cell_height,
                            cell_width,
                            cell_height,
                        ],
                        transform,
                        g,
                    );
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let cells = make_cells();

    let thread2_cells = cells.clone();
    std::thread::spawn(move || {
        trace_rays(thread2_cells);
    });

    while let Some(e) = window.next() {
        let size = window.size();
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);

            render_screen(&cells, size, c.transform, g);
        });
    }
}

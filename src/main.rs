extern crate piston_window;
use piston_window::*;
use std::sync::{Arc, Mutex};

const CELLS_WIDE: usize = 60;
const CELLS_HIGH: usize = 60;

type Color = [f32; 4];

type Cells = [Color; CELLS_HIGH * CELLS_WIDE];

fn trace() -> Color {
    [1.0, 0.1, 0.8, 1.0]
}

fn get_xy(index: usize) -> (usize, usize) {
    (index / CELLS_WIDE, index % CELLS_WIDE)
}

fn get_index(x: usize, y: usize) -> usize {
    x * CELLS_WIDE + y
}

fn trace_rays(cells: Arc<Mutex<Box<Cells>>>, range: std::ops::Range<usize>) {
    std::thread::spawn(move || {
        for index in range {
            std::thread::sleep(std::time::Duration::new(0, 500_000_000));
            cells.lock().unwrap()[index] = trace();
        }
    });
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let cells = Arc::new(Mutex::new(Box::new(
        [[0.1, 0.1, 0.1, 1.0]; CELLS_HIGH * CELLS_WIDE],
    )));

    trace_rays(cells.clone(), 0..(CELLS_HIGH * CELLS_WIDE));

    while let Some(e) = window.next() {
        let s = window.size();
        let cell_width = s.width as f64 / CELLS_WIDE as f64;
        let cell_height = s.height as f64 / CELLS_HIGH as f64;
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);

            for x in 0..CELLS_WIDE {
                for y in 0..CELLS_HIGH {
                    rectangle(
                        cells.lock().unwrap()[get_index(x, y)],
                        [
                            x as f64 * cell_width,
                            y as f64 * cell_height,
                            cell_width,
                            cell_height,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        });
    }
}

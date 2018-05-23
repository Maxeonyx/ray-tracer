extern crate cgmath;
extern crate piston_window;
extern crate rayon;
use piston_window::*;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

mod shapes;
mod solvers;
mod types;

use types::*;

use shapes::*;

const DEFAULT_COLOR: Color = [0.1, 0.1, 0.1, 1.0];
const BACKGROUND_COLOR: Color = [0.8, 0.1, 0.8, 1.0];

fn make_cells() -> Cells {
    let mut v = Vec::with_capacity(CELLS_HIGH * CELLS_WIDE);
    for _ in 0..(CELLS_HIGH * CELLS_WIDE) {
        v.push(Mutex::new(DEFAULT_COLOR));
    }
    Arc::new(v)
}

fn trace(ray: &Ray, scene: &Scene) -> Color {
    use std::cmp::Ordering;

    let intersections = scene.into_iter().filter_map(move |obj| {
        match obj.closest_intersection(ray) {
            None => None,
            Some(t) => Some((t, obj)),
        }
    });

    let closest_intersect =
        intersections.min_by(|(t1, _), (t2, _)| t1.partial_cmp(t2).unwrap_or(Ordering::Equal));

    match closest_intersect {
        None => BACKGROUND_COLOR,
        Some((t, obj)) => obj.color(ray, t),
    }
}

fn get_xy(index: usize) -> (usize, usize) {
    (index / CELLS_WIDE, index % CELLS_WIDE)
}

fn get_index(x: usize, y: usize) -> usize {
    x * CELLS_WIDE + y
}

fn trace_rays(cells: Cells, scene: Scene) {
    use rayon::prelude::*;

    cells.par_iter().enumerate().for_each(|(index, cell)| {
        //std::thread::sleep(std::time::Duration::new(0, 500_000_000));

        let (camera_sensor_width, camera_sensor_height, camera_sensor_dist) = (1.0, 1.0, 0.5);

        let (cell_x, cell_y) = get_xy(index);
        let (cell_x, cell_y) = (cell_x as f32, cell_y as f32);

        let ray = Ray {
            origin: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: V3 {
                x: -camera_sensor_width / 2.0 + cell_x * (camera_sensor_width / CELLS_WIDE as f32),
                y: -camera_sensor_height / 2.0
                    + cell_y * (camera_sensor_height / CELLS_HIGH as f32),
                z: camera_sensor_dist,
            },
        };

        let color = trace(&ray, &scene);

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
                _ => println!("Couldn't read to render screen: hit lock"),
            }
        }
    }
}

type Scene = Vec<Box<Object2>>;

fn initialise_scene() -> Scene {
    vec![
        Box::new(Object2 {
            position: V3 {
                x: 1.0,
                y: 1.0,
                z: 5.0,
            },
            color: [0.1, 0.8, 0.1, 1.0],
            shape: Shape::Sphere(1.0),
            surface: Surface::Diffuse,
        }),
    ]
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", (CELLS_HIGH as u32, CELLS_WIDE as u32))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let cells = make_cells();

    let thread2_cells = cells.clone();
    std::thread::spawn(move || {
        let scene: Scene = initialise_scene();

        trace_rays(thread2_cells, scene);
    });

    while let Some(e) = window.next() {
        let size = window.size();
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);

            render_screen(&cells, size, c.transform, g);
        });
    }
}

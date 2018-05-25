extern crate cgmath;
extern crate piston_window;
extern crate rayon;
use cgmath::prelude::*;
use piston_window::*;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

mod shapes;
mod solvers;
mod types;

use types::*;

use shapes::*;

const DEFAULT_COLOR: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
};
const BACKGROUND_COLOR: Color = Color {
    r: 0.8,
    g: 0.1,
    b: 0.8,
};

const MAX_TRACE_DEPTH: u32 = 12;

fn make_cells() -> Cells {
    let mut v = Vec::with_capacity(CELLS_HIGH * CELLS_WIDE);
    for _ in 0..(CELLS_HIGH * CELLS_WIDE) {
        v.push(Mutex::new(DEFAULT_COLOR));
    }
    Arc::new(v)
}

fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth > MAX_TRACE_DEPTH {
        return BACKGROUND_COLOR;
    }

    let intersections = scene.into_iter().filter_map(move |obj| {
        match obj.closest_intersection(ray) {
            None => None,
            Some(t) => Some((t, obj)),
        }
    });

    use std::cmp::Ordering;
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

        let mut ray = Ray {
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

        ray.direction = ray.direction.normalize();

        let color = trace(&ray, &scene, 0);

        'try_update: loop {
            match cell.lock() {
                Ok(mut c) => {
                    *c = color;
                    break 'try_update;
                }
                Err(_) => println!("Hit lock."),
            }
        }
    });
}

fn render_screen<G>(cells: Cells, screen_size: Size, transform: [[f64; 3]; 2], g: &mut G)
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
                        [cell.r, cell.g, cell.b, 1.0],
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
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            color: Color {
                r: 0.1,
                g: 0.8,
                b: 0.1,
            },
            shape: Shape::Sphere(1.0),
            surface: Surface::Diffuse,
        }),
    ]
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston!",
        Size {
            width: 400,
            height: 600,
        },
    ).exit_on_esc(true)
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
        let closure_cells = cells.clone();
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);

            render_screen(closure_cells, size, c.transform, g);
        });
    }
}

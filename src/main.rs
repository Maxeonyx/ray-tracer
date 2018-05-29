extern crate cgmath;
#[macro_use]
extern crate glium;
//extern crate piston_window;
extern crate rayon;
use cgmath::prelude::*;
// use piston_window::*;
use std::cmp::Ordering;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use glium::index::PrimitiveType;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{glutin, Surface};
mod shapes;
mod types;
mod util;

use shapes::*;
use types::*;
use util::V3Extensions;

const DEFAULT_COLOR: Color = V3 {
    x: 0.0,
    y: 0.0,
    z: 0.01,
};
const BACKGROUND_COLOR: Color = Color {
    x: 0.1,
    y: 0.1,
    z: 0.1,
};

const MAX_TRACE_DEPTH: u32 = 12;

fn make_cells() -> Cells {
    let mut v = Vec::with_capacity(CELLS_HIGH * CELLS_WIDE);
    for _ in 0..(CELLS_HIGH * CELLS_WIDE) {
        v.push(Mutex::new(DEFAULT_COLOR));
    }
    Cells { data: Arc::new(v) }
}

fn closest_intersect<'a>(ray: &Ray, scene: &'a Scene) -> Option<(f32, &'a Object2)> {
    scene
        .objects
        .iter()
        .filter_map(move |obj| match obj.closest_intersection(ray) {
            None => None,
            Some(t) => Some((t, obj)),
        })
        .min_by(|(t1, _obj1), (t2, _obj2)| t1.partial_cmp(t2).unwrap_or(Ordering::Equal))
}

fn trace_shadow(point: V3, light: &Light, scene: &Scene) -> bool {
    let shadow_ray = Ray {
        origin: point,
        direction: (light.position - point).normalize(),
    };
    let closest_intersect = closest_intersect(&shadow_ray, &scene);

    match closest_intersect {
        None => false,
        Some((t, _obj)) => {
            // light may be closer than object
            if (light.position - point).magnitude() < t {
                false
            } else {
                true
            }
        }
    }
}

fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth > MAX_TRACE_DEPTH {
        return BACKGROUND_COLOR;
    }

    let closest_intersect = closest_intersect(&ray, &scene);

    match closest_intersect {
        None => BACKGROUND_COLOR,
        Some((t, obj)) => {
            let intersect = ray.direction * t;

            let normal = obj.normal(intersect);

            let total_brightness = scene
                .lights
                .iter()
                .map(|light| light.brightness)
                .sum::<f32>();

            let diffuse_factor: f32 = scene
                .lights
                .iter()
                .map(|light| -> f32 {
                    let light_vec = (light.position - intersect).normalize();
                    if trace_shadow(intersect, light, scene) {
                        0.0
                    } else {
                        light_vec.dot(normal).max(0.0) * light.brightness / total_brightness
                    }
                })
                .sum::<f32>();

            let specular_factor: f32 = scene
                .lights
                .iter()
                .map(|light| -> f32 {
                    let light_vec = (light.position - intersect).normalize();
                    let reflected = (-light_vec).reflect(normal);
                    let rdotn = reflected.dot(normal);
                    let shininess = 20.0;
                    if trace_shadow(intersect, light, scene) {
                        0.0
                    } else {
                        rdotn.max(0.0).powf(shininess) * light.brightness / total_brightness
                    }
                })
                .sum::<f32>();

            0.1 * obj.color + diffuse_factor * obj.color + specular_factor * obj.color
        }
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

    cells.data.par_iter().enumerate().for_each(|(index, cell)| {
        //std::thread::sleep(std::time::Duration::new(0, 500_000_000));

        let (camera_sensor_width, camera_sensor_height, camera_sensor_dist) = (1.0, 1.0, 0.5);

        let (cell_x, cell_y) = get_xy(index);
        let (cell_x, cell_y) = (cell_x as f32, cell_y as f32);

        let mut colors = Vec::new();

        let antialiasing_div_size = 1.0 / (ANTIALIASING_DIV as f32);

        for x in 0..ANTIALIASING_DIV {
            for y in 0..ANTIALIASING_DIV {
                let x_offset = x as f32 * antialiasing_div_size + antialiasing_div_size / 2.0;
                let y_offset = y as f32 * antialiasing_div_size + antialiasing_div_size / 2.0;

                let mut ray = Ray {
                    origin: V3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    direction: V3 {
                        x: -camera_sensor_width / 2.0
                            + (cell_x + x_offset) * (camera_sensor_width / CELLS_WIDE as f32),
                        y: -camera_sensor_height / 2.0
                            + (cell_y + y_offset) * (camera_sensor_height / CELLS_HIGH as f32),
                        z: -camera_sensor_dist,
                    },
                };
                ray.direction = ray.direction.normalize();
                colors.push(trace(&ray, &scene, 0));
            }
        }

        let color = colors.iter().sum::<V3>() / colors.len() as f32;

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

pub struct Scene {
    objects: Vec<Object2>,
    lights: Vec<Light>,
}

fn initialise_scene() -> Scene {
    Scene {
        objects: vec![
            Object2 {
                color: V3 {
                    x: 0.1,
                    y: 0.8,
                    z: 0.1,
                },
                shape: Shape::Sphere(Sphere {
                    center: V3 {
                        x: 0.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    radius: 1.0,
                }),
                surface: shapes::Surface::Diffuse,
            },
            Object2 {
                color: V3 {
                    x: 0.3,
                    y: 0.3,
                    z: 0.6,
                },
                shape: Shape::Triangle(Triangle {
                    vertex_1: V3 {
                        x: 0.0,
                        y: -1.0,
                        z: -6.0,
                    },
                    vertex_2: V3 {
                        x: 0.0,
                        y: -3.0,
                        z: -6.0,
                    },
                    vertex_3: V3 {
                        x: 3.0,
                        y: -3.5,
                        z: -12.0,
                    },
                }),
                surface: shapes::Surface::Diffuse,
            },
            Object2 {
                color: V3 {
                    x: 0.8,
                    y: 0.4,
                    z: 0.1,
                },
                shape: Shape::Sphere(Sphere {
                    center: V3 {
                        x: 0.0,
                        y: 3.0,
                        z: -10.0,
                    },
                    radius: 10.0,
                }),
                surface: shapes::Surface::Diffuse,
            },
        ],
        lights: vec![
            Light {
                position: V3 {
                    x: 6.0,
                    y: -6.0,
                    z: 0.0,
                },
                brightness: 10.0,
            },
            Light {
                position: V3 {
                    x: -6.0,
                    y: -6.0,
                    z: 0.0,
                },
                brightness: 3.0,
            },
        ],
    }
}

fn main() {
    // building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let cells = make_cells();

    let thread2_cells = cells.clone();
    std::thread::spawn(move || {
        let scene: Scene = initialise_scene();

        trace_rays(thread2_cells, scene);
    });

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                },
            ],
        ).unwrap()
    };

    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3])
            .unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                in vec2 position;
                out vec2 asdf_position;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                    asdf_position = position / 2 + vec2(0.5, 0.5);
                }
            ",

            fragment: "
                #version 140
                out vec4 f_color;
                in vec2 asdf_position;
                layout(std140) uniform;
                uniform vec2 divisions;

                uniform sampler2D cells;

                void main() {

                    f_color = texture(cells, asdf_position);
                }
            "
        },
    ).unwrap();

    // the main loop
    let mut closed = false;
    while !closed {
        let cells_image = glium::texture::RawImage2d::from_raw_rgba(
            cells.clone().to_vec(),
            (CELLS_WIDE as u32, CELLS_HIGH as u32),
        );
        let cells_texture = glium::texture::Texture2d::new(&display, cells_image).unwrap();
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 0.0);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniform! {
                    divisions: [CELLS_WIDE as f32, CELLS_HIGH as f32],
                    cells: &cells_texture,
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        // polling and handling the events received by the window
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}

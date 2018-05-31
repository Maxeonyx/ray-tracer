extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;
extern crate rayon;

use cgmath::prelude::*;
use glium::index::PrimitiveType;
use glium::{glutin, Surface};
use image::GenericImage;
use image::{load_from_memory, DynamicImage};
use rand::*;
use std::cmp::Ordering;
use std::iter::Iterator;
use std::sync::Arc;
use std::vec::Vec;

mod scene;
mod shapes;
mod types;
mod util;

use scene::Scene;
use shapes::*;
use types::*;
use util::V3Extensions;

fn make_cells() -> Cells {
    let mut v = Vec::with_capacity(CELLS_HIGH * CELLS_WIDE);
    for _ in 0..(CELLS_HIGH * CELLS_WIDE) {
        v.push(Cell::new(DEFAULT_COLOR));
    }
    Cells { data: Arc::new(v) }
}

fn closest_intersect<'a>(ray: &Ray, scene: &'a Scene) -> Option<(f32, &'a Object2)> {
    scene
        .objects()
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

fn trace(ray: &Ray, scene: &Scene, textures: &Vec<DynamicImage>, depth: u32) -> Color {
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
                .lights()
                .iter()
                .map(|light| light.brightness)
                .sum::<f32>();

            let diffuse_factor: f32 = scene
                .lights()
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
                .lights()
                .iter()
                .map(|light| -> f32 {
                    let light_vec = (light.position - intersect).normalize();
                    let reflected = (-light_vec).reflect(normal);
                    let rdotn = reflected.dot(normal);
                    let shininess = 50.0;
                    if trace_shadow(intersect, light, scene) {
                        0.0
                    } else {
                        rdotn.max(0.0).powf(shininess) * light.brightness / total_brightness
                    }
                })
                .sum::<f32>();

            let surface_color = diffuse_factor * obj.color + specular_factor * obj.color;

            let surface_color = match obj.surface {
                shapes::Surface::Diffuse => surface_color,
                shapes::Surface::Reflective(portion) => {
                    let reflected_ray = Ray {
                        direction: ray.direction.reflect(normal),
                        origin: intersect,
                    };

                    let reflected_color = trace(&reflected_ray, scene, textures, depth + 1);

                    reflected_color * portion + surface_color * (1.0 - portion)
                }
                shapes::Surface::Textured(texture) => {
                    let texture_coord = obj.get_texture_coord(intersect);

                    let texture = &textures[texture];
                    let (width, height) = texture.dimensions();

                    let pixel_x = width * (texture_coord.x % 1.0) as u32;
                    let pixel_y = height * (texture_coord.y % 1.0) as u32;

                    let pixel = texture.get_pixel(pixel_x, pixel_y);

                    V3 {
                        x: pixel.data[0] as f32 / 255.0,
                        y: pixel.data[1] as f32 / 255.0,
                        z: pixel.data[2] as f32 / 255.0,
                    }
                }
            };

            0.1 * obj.color + surface_color
        }
    }
}

fn get_xy(index: usize) -> (usize, usize) {
    (index % CELLS_WIDE, index / CELLS_WIDE)
}

fn trace_rays(cells: Cells, textures: &Vec<DynamicImage>, scene: Scene) {
    use rayon::prelude::*;

    let mut range: Vec<usize> = (0..(CELLS_HIGH * CELLS_WIDE)).collect();
    let range = range.as_mut_slice();
    //thread_rng().shuffle(range);

    let y_rotation = cgmath::Quaternion::from_arc(
        V3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        V3 {
            x: 0.0,
            y: 2.0,
            z: -1.0,
        },
        None,
    );

    range.into_par_iter().for_each(|index| {
        let (camera_sensor_width, camera_sensor_height, camera_sensor_dist) = (1.0, 1.0, 0.5);

        let (cell_x, cell_y) = get_xy(*index);
        let (cell_x, cell_y) = (cell_x as f32, cell_y as f32);

        let mut colors = [V3::zero(); ANTIALIASING_DIV * ANTIALIASING_DIV];

        let antialiasing_div_size = 1.0 / (ANTIALIASING_DIV as f32);

        for x in 0..ANTIALIASING_DIV {
            for y in 0..ANTIALIASING_DIV {
                let x_offset = x as f32 * antialiasing_div_size + antialiasing_div_size / 2.0;
                let y_offset = y as f32 * antialiasing_div_size + antialiasing_div_size / 2.0;

                let ray_direction = V3 {
                    x: -camera_sensor_width / 2.0
                        + (cell_x + x_offset) * (camera_sensor_width / CELLS_WIDE as f32),
                    y: -camera_sensor_height / 2.0
                        + (cell_y + y_offset) * (camera_sensor_height / CELLS_HIGH as f32),
                    z: -camera_sensor_dist,
                };

                //let ray_direction = y_rotation.rotate_vector(ray_direction);

                let mut ray = Ray {
                    origin: V3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    direction: ray_direction,
                };
                ray.direction = ray.direction.normalize();
                colors[x * ANTIALIASING_DIV + y] = trace(&ray, &scene, textures, 0);
            }
        }

        let color = colors.iter().sum::<V3>() / colors.len() as f32;

        let cell = &cells.data[*index];

        cell.set_content(color)
    });
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
        let mut textures = vec![];
        let scene = Scene::initialise(&mut textures);

        trace_rays(thread2_cells, &textures, scene);
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
                    position: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                },
                Vertex {
                    position: [-1.0, -1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
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
            vertex: include_str!("shaders/main.vert.glsl"),
            fragment: include_str!("shaders/main.frag.glsl"),
        },
    ).unwrap();

    // the main loop
    let mut jessica = false;
    while !jessica {
        let frame_deadline =
            std::time::Instant::now() + std::time::Duration::from_millis(1_000 / 60);

        let cells_image = glium::texture::RawImage2d::from_raw_rgb(
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
                glutin::WindowEvent::Closed => jessica = true,
                _ => (),
            },
            _ => (),
        });
        let time = std::time::Instant::now();
        if time < frame_deadline {
            std::thread::sleep(frame_deadline - time);
        }
    }
}

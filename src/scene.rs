use shapes::*;
use types::*;

pub struct Scene {
    objects: Vec<Object2>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn objects(&self) -> &Vec<Object2> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn initialise() -> Scene {
        let mut objects = vec![];
        objects.append(&mut make_cube(
            true,
            V3 {
                x: 0.6,
                y: 0.7,
                z: 0.2,
            },
            Surface::Diffuse,
        ));

        objects.push(Object2 {
            shape: Shape::Sphere(Sphere {
                center: V3 {
                    x: 1.2,
                    y: -0.9,
                    z: -6.0,
                },
                radius: 1.0,
            }),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.9,
                y: 0.1,
                z: 0.0,
            },
        });

        let mut lights = vec![Light {
            position: V3 {
                x: -4.2,
                y: -5.0,
                z: -0.5,
            },
            brightness: 100.0,
        }];

        Scene { objects, lights }
    }
}
const VERTEX_ORDER: [usize; 14] = [7, 5, 1, 3, 2, 5, 4, 7, 6, 1, 0, 2, 6, 4];

const CUBE_WIDTH: f32 = 2.6;

const CUBE_VERTICES: [V3; 8] = [
    //0
    V3 {
        x: -CUBE_WIDTH,
        y: CUBE_WIDTH,
        z: -7.0,
    },
    //1
    V3 {
        x: -CUBE_WIDTH,
        y: CUBE_WIDTH,
        z: -4.0,
    },
    //2
    V3 {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -7.0,
    },
    //3
    V3 {
        x: -CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -4.0,
    },
    //4
    V3 {
        x: CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -7.0,
    },
    //5
    V3 {
        x: CUBE_WIDTH,
        y: -CUBE_WIDTH,
        z: -4.0,
    },
    //6
    V3 {
        x: CUBE_WIDTH,
        y: CUBE_WIDTH,
        z: -7.0,
    },
    //7
    V3 {
        x: CUBE_WIDTH,
        y: CUBE_WIDTH,
        z: -4.0,
    },
];

fn make_cube(open_top: bool, color: Color, surface: Surface) -> Vec<Object2> {
    let mut triangles = Vec::new();

    for i in 0..12 {
        if open_top && (i == 0 || i == 1) {
            continue;
        }
        let indices = &VERTEX_ORDER[i..i + 3];
        let triangle = if i % 2 == 0 {
            Triangle::new([
                CUBE_VERTICES[indices[0]],
                CUBE_VERTICES[indices[1]],
                CUBE_VERTICES[indices[2]],
            ])
        } else {
            Triangle::new([
                CUBE_VERTICES[indices[0]],
                CUBE_VERTICES[indices[2]],
                CUBE_VERTICES[indices[1]],
            ])
        };
        triangles.push(Object2 {
            shape: Shape::Triangle(triangle),
            // color: V3 {
            //     x: ::rand::random::<f32>() % 1.0,
            //     y: ::rand::random::<f32>() % 1.0,
            //     z: ::rand::random::<f32>() % 1.0,
            // },
            color,
            surface,
        });
    }
    triangles
}

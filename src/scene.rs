use image::{load_from_memory, DynamicImage};
use shapes::*;
use types::*;

pub struct Scene {
    objects: Vec<Object2>,
    lights: Vec<Light>,
}

const CAT_IMAGE_BYTES: &[u8] = include_bytes!("images/CUTE-CAT.jpg");
const CARPET_IMAGE_BYTES: &[u8] = include_bytes!("images/seamless_carpet_texture.jpg");

use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::Quaternion;
use cgmath::Rotation3;

fn transform(input: V3) -> V3 {
    let scene_center = V3 {
        x: 17.0,
        y: 20.0,
        z: -20.0,
    };

    let rotation_y = Quaternion::from_arc(
        V3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        V3 {
            x: 0.0,
            y: -1.8,
            z: -1.0,
        },
        None,
    );

    let rotation_x = Quaternion::from_arc(
        V3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        V3 {
            x: 1.0,
            y: 0.85,
            z: 0.0,
        },
        None,
    );

    let output = input;

    let output = output + scene_center;

    let output = rotation_x.rotate_vector(output);

    let output = rotation_y.rotate_vector(output);

    output
}

impl Scene {
    pub fn objects(&self) -> &Vec<Object2> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn initialise(textures: &mut Vec<DynamicImage>) -> Scene {
        textures.push(::image::load_from_memory(CAT_IMAGE_BYTES).unwrap());
        textures.push(::image::load_from_memory(CARPET_IMAGE_BYTES).unwrap());

        let mut objects = vec![];
        objects.append(&mut make_cube(
            true,
            V3 {
                x: 0.9,
                y: 0.5,
                z: 0.0,
            },
            Surface::Diffuse,
        ));

        // orange sphere
        objects.push(Object2 {
            shape: Shape::Sphere(Sphere {
                center: transform(V3 {
                    x: -5.0,
                    y: -5.0,
                    z: 3.0,
                }),
                radius: 6.5,
            }),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.9,
                y: 0.1,
                z: 0.0,
            },
            shininess: 80.0,
        });

        // green sphere
        objects.push(Object2 {
            shape: Shape::Sphere(Sphere {
                center: transform(V3 {
                    x: -1.0,
                    y: -16.0,
                    z: 3.0,
                }),
                radius: 3.0,
            }),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.0,
                y: 1.0,
                z: 0.3,
            },
            shininess: 40.0,
        });

        // reflective blue sphere
        objects.push(Object2 {
            shape: Shape::Sphere(Sphere {
                center: transform(V3 {
                    x: 6.0,
                    y: 0.0,
                    z: 3.0,
                }),
                radius: 15.0,
            }),
            surface: Surface::Reflective(0.95),
            color: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            shininess: 40.0,
        });

        let cat_triangle_vertices = [
            transform(V3 {
                x: -5.0,
                y: 27.5,
                z: 30.0,
            }),
            transform(V3 {
                x: -5.0,
                y: 27.5,
                z: 0.0,
            }),
            transform(V3 {
                x: 17.0,
                y: 27.5,
                z: 0.0,
            }),
            transform(V3 {
                x: 17.0,
                y: 27.5,
                z: 30.0,
            }),
        ];

        // cat triangle 1
        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new_with_uv(
                [
                    cat_triangle_vertices[0],
                    cat_triangle_vertices[1],
                    cat_triangle_vertices[2],
                ],
                [
                    V2 { x: 0.0, y: 0.0 },
                    V2 { x: 0.0, y: 1.0 },
                    V2 { x: 1.0, y: 1.0 },
                ],
            )),
            surface: Surface::Textured(0),
            color: V3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            shininess: 40.0,
        });

        // cat triangle 2
        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new_with_uv(
                [
                    cat_triangle_vertices[0],
                    cat_triangle_vertices[2],
                    cat_triangle_vertices[3],
                ],
                [
                    V2 { x: 0.0, y: 0.0 },
                    V2 { x: 1.0, y: 1.0 },
                    V2 { x: 1.0, y: 0.0 },
                ],
            )),
            surface: Surface::Textured(0),
            color: V3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            shininess: 40.0,
        });

        let carpet_triangle_vertices = [
            transform(V3 {
                x: 0.0,
                y: 400.0,
                z: -0.001,
            }),
            transform(V3 {
                x: 400.0,
                y: 0.0,
                z: -0.001,
            }),
            transform(V3 {
                x: 0.0,
                y: -400.0,
                z: -0.001,
            }),
            transform(V3 {
                x: -400.0,
                y: 0.0,
                z: -0.001,
            }),
        ];

        let carpet_wrap_factor = 20.0;

        // carpet triangle 1
        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new_with_uv(
                [
                    carpet_triangle_vertices[0],
                    carpet_triangle_vertices[1],
                    carpet_triangle_vertices[2],
                ],
                [
                    V2 { x: 0.0, y: 0.0 },
                    V2 {
                        x: 0.0,
                        y: carpet_wrap_factor,
                    },
                    V2 {
                        x: carpet_wrap_factor,
                        y: carpet_wrap_factor,
                    },
                ],
            )),
            surface: Surface::Textured(1),
            color: V3 {
                x: 0.4,
                y: 0.1,
                z: 0.05,
            },
            shininess: 0.0,
        });

        // carpet triangle 2
        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new_with_uv(
                [
                    carpet_triangle_vertices[0],
                    carpet_triangle_vertices[2],
                    carpet_triangle_vertices[3],
                ],
                [
                    V2 { x: 0.0, y: 0.0 },
                    V2 {
                        x: carpet_wrap_factor,
                        y: carpet_wrap_factor,
                    },
                    V2 {
                        x: carpet_wrap_factor,
                        y: 0.0,
                    },
                ],
            )),
            surface: Surface::Textured(1),
            color: V3 {
                x: 0.4,
                y: 0.1,
                z: 0.05,
            },
            shininess: 0.0,
        });

        let tetrahedron_pos = V3 {
            x: -3.0,
            y: 10.0,
            z: 3.0,
        };

        let tetrahedron_rotation = Quaternion::from_axis_angle(V3::unit_z(), Deg(60.0));

        let tetrahedron_size = 4.0;

        let tetrahedron_vertices = [
            transform(
                tetrahedron_pos
                    + tetrahedron_rotation.rotate_vector(
                        V3 {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0 / 2.0_f32.sqrt(),
                        } * tetrahedron_size,
                    ),
            ),
            transform(
                tetrahedron_pos
                    + tetrahedron_rotation.rotate_vector(
                        V3 {
                            x: -1.0,
                            y: 0.0,
                            z: 1.0 / 2.0_f32.sqrt(),
                        } * tetrahedron_size,
                    ),
            ),
            transform(
                tetrahedron_pos
                    + tetrahedron_rotation.rotate_vector(
                        V3 {
                            x: 0.0,
                            y: 1.0,
                            z: -1.0 / 2.0_f32.sqrt(),
                        } * tetrahedron_size,
                    ),
            ),
            transform(
                tetrahedron_pos
                    + tetrahedron_rotation.rotate_vector(
                        V3 {
                            x: 0.0,
                            y: -1.0,
                            z: -1.0 / 2.0_f32.sqrt(),
                        } * tetrahedron_size,
                    ),
            ),
        ];

        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new([
                tetrahedron_vertices[0],
                tetrahedron_vertices[1],
                tetrahedron_vertices[2],
            ])),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.2,
                y: 0.7,
                z: 0.4,
            },
            shininess: 10.0,
        });

        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new([
                tetrahedron_vertices[0],
                tetrahedron_vertices[1],
                tetrahedron_vertices[3],
            ])),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.2,
                y: 0.7,
                z: 0.4,
            },
            shininess: 10.0,
        });

        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new([
                tetrahedron_vertices[0],
                tetrahedron_vertices[2],
                tetrahedron_vertices[3],
            ])),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.2,
                y: 0.7,
                z: 0.4,
            },
            shininess: 10.0,
        });

        objects.push(Object2 {
            shape: Shape::Triangle(Triangle::new([
                tetrahedron_vertices[1],
                tetrahedron_vertices[2],
                tetrahedron_vertices[3],
            ])),
            surface: Surface::Diffuse,
            color: V3 {
                x: 0.2,
                y: 0.7,
                z: 0.4,
            },
            shininess: 10.0,
        });

        let mut lights = vec![
            Light {
                position: transform(V3 {
                    x: -29.0,
                    y: -10.0,
                    z: 13.0,
                }),
                brightness: 40.0,
            },
            Light {
                position: transform(V3 {
                    x: 25.0,
                    y: 19.0,
                    z: 19.0,
                }),
                brightness: 50.0,
            },
            Light {
                position: transform(V3 {
                    x: 0.0,
                    y: -29.0,
                    z: 19.0,
                }),
                brightness: 60.0,
            },
        ];

        Scene { objects, lights }
    }
}

const VERTEX_ORDER: [usize; 14] = [7, 5, 1, 3, 2, 5, 4, 7, 6, 1, 0, 2, 6, 4];

const CUBE_WIDTH: f32 = 20.5;
const CUBE_HEIGHT: f32 = 7.0;

const CUBE: V3 = V3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const CUBE_VERTICES: [V3; 8] = [
    //0
    V3 {
        x: -CUBE_WIDTH,
        y: 0.0,
        z: 0.0,
    },
    //1
    V3 {
        x: 0.0,
        y: -CUBE_WIDTH,
        z: 0.0,
    },
    //2
    V3 {
        x: -CUBE_WIDTH,
        y: 0.0,
        z: CUBE_HEIGHT,
    },
    //3
    V3 {
        x: 0.0,
        y: -CUBE_WIDTH,
        z: CUBE_HEIGHT,
    },
    //4
    V3 {
        x: 0.0,
        y: CUBE_WIDTH,
        z: CUBE_HEIGHT,
    },
    //5
    V3 {
        x: CUBE_WIDTH,
        y: 0.0,
        z: CUBE_HEIGHT,
    },
    //6
    V3 {
        x: 0.0,
        y: CUBE_WIDTH,
        z: 0.0,
    },
    //7
    V3 {
        x: CUBE_WIDTH,
        y: 0.0,
        z: 0.0,
    },
];

fn make_cube(open_top: bool, color: Color, surface: Surface) -> Vec<Object2> {
    let mut triangles = Vec::new();

    for i in 0..12 {
        if open_top && (i == 3 || i == 4) {
            continue;
        }

        let flip_normal = if (i == 2 || i == 9) { true } else { false };

        let indices = &VERTEX_ORDER[i..i + 3];

        let odd_index = i % 2 == 1;

        let odd_index = if flip_normal { !odd_index } else { odd_index };

        let triangle = if odd_index {
            Triangle::new([
                transform(CUBE_VERTICES[indices[0]] + CUBE),
                transform(CUBE_VERTICES[indices[1]] + CUBE),
                transform(CUBE_VERTICES[indices[2]] + CUBE),
            ])
        } else {
            Triangle::new([
                transform(CUBE_VERTICES[indices[0]] + CUBE),
                transform(CUBE_VERTICES[indices[2]] + CUBE),
                transform(CUBE_VERTICES[indices[1]] + CUBE),
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
            shininess: 20.0,
        });
    }
    triangles
}

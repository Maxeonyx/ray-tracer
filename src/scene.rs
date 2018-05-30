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

        let ball = Shape::Sphere(Sphere {
            center: V3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 4.0,
        });

        Scene {
            objects: vec![Object2 {
                color: V3 {
                    x: 0.6,
                    y: 0.6,
                    z: 0.9,
                },
                surface: Surface::Diffuse,
                shape: ball,
            }],
            lights: vec![Light {
                position: V3 {
                    x: -2.0,
                    y: 3.0,
                    z: -0.5,
                },
                brightness: 100.0,
            }],
        }
    }
}

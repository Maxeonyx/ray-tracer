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
                    surface: Surface::Diffuse,
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
                    surface: Surface::Diffuse,
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
                    surface: Surface::Diffuse,
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
}

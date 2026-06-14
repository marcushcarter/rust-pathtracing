mod camera;
mod sphere;
mod triangle;

pub use camera::{Camera, CameraData};
pub use sphere::Sphere;
pub use triangle::Triangle;

use nalgebra_glm as glm;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
}

impl Scene {

    pub fn demo() -> Self {
        let spheres = vec![
            Sphere::glass(glm::vec3(-2.25, 0.0, 0.0), 0.5, 1.5),
            Sphere::diffuse(glm::vec3(-0.75, 0.0, 0.0), 0.5, glm::vec3(1.0, 1.0, 1.0)),
            Sphere::metal(glm::vec3(0.75, 0.0, 0.0), 0.5, glm::vec3(1.0, 1.0, 1.0), 0.0),
            Sphere::metal(glm::vec3(2.25, 0.0, 0.0), 0.5, glm::vec3(0.5, 0.5, 1.0), 0.3),
        ];

        let y = -0.5;
        let h = 5.0;
        let c0 = glm::vec3(-h, y, -h);
        let c1 = glm::vec3( h, y, -h);
        let c2 = glm::vec3( h, y,  h);
        let c3 = glm::vec3(-h, y,  h);
        let ground = glm::vec3(0.7, 0.7, 0.7);

        let triangles = vec![
            Triangle::diffuse(c0, c1, c2, ground),
            Triangle::diffuse(c0, c2, c3, ground),
        ];

        Scene { spheres, triangles }
    }
}
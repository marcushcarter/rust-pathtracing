use nalgebra_glm as glm;

#[repr(C)]
pub struct Triangle {
    pub v0: glm::Vec3,
    pub _p0: f32,
    pub v1: glm::Vec3,
    pub _p1: f32,
    pub v2: glm::Vec3,
    pub _p2: f32,
    pub albedo: glm::Vec3,
    pub fuzz: f32,
    pub mat_type: i32,
    pub ior: f32,
    pub _pad: [f32;2],
}

impl Triangle {
    pub fn diffuse(v0: glm::Vec3, v1: glm::Vec3, v2: glm::Vec3, albedo: glm::Vec3) -> Self {
        Self {
            v0, _p0: 0.0, v1, _p1: 0.0, v2, _p2: 0.0, albedo, fuzz: 0.0, mat_type: 0, ior: 0.0, _pad: [0.0; 2],
        }
    }

    // pub fn metal(v0: glm::Vec3, v1: glm::Vec3, v2: glm::Vec3, albedo: glm::Vec3, fuzz: f32) -> Self {
    //     Self {
    //         v0, _p0: 0.0, v1, _p1: 0.0, v2, _p2: 0.0, albedo, fuzz: fuzz.clamp(0.0, 1.0), mat_type: 1, ior: 0.0, _pad: [0.0; 2],
    //     }
    // }
}

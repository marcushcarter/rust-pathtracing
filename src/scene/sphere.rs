use nalgebra_glm as glm;

#[repr(C)]
pub struct Sphere {
    pub center: glm::Vec3,
    pub radius: f32,
    pub albedo: glm::Vec3,
    pub fuzz: f32,
    pub mat_type: i32,
    pub ior: f32,
    pub _pad: [f32; 2],
}

impl Sphere {
    pub fn diffuse(center: glm::Vec3, radius: f32, albedo: glm::Vec3) -> Self {
        Self {
            center, radius, albedo, fuzz: 0.0, mat_type: 0, ior: 0.0, _pad: [0.0; 2],
        }
    }

    pub fn metal(center: glm::Vec3, radius: f32, albedo: glm::Vec3, fuzz: f32) -> Self {
        Self {
            center, radius, albedo, fuzz: fuzz.clamp(0.0, 1.0), mat_type: 1, ior: 0.0, _pad: [0.0; 2],
        }
    }
    
    pub fn glass(center: glm::Vec3, radius: f32, ior: f32) -> Self {
        Self {
            center, radius, albedo: glm::vec3(1.0, 1.0, 1.0), fuzz: 0.0, mat_type: 2, ior, _pad: [0.0; 2],
        }
    }
}
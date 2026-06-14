use nalgebra_glm as glm;

pub struct Camera {
    pub target: glm::Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,

    pub min_distance: f32,
    pub max_distance: f32,
    pub max_pitch: f32,
}

#[repr(C)]
pub struct CameraData {
    pub pos: glm::Vec3,
    pub tan_half_fov: f32,
    pub forward: glm::Vec3,
    pub _pad0: f32,
    pub right: glm::Vec3,
    pub _pad1: f32,
    pub up: glm::Vec3,
    pub _pad2: f32,
    pub resolution: glm::Vec2,
    pub frame: u32,
    pub sample_count: u32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: glm::vec3(0.0, 0.25, 0.0),
            distance: 4.0,
            yaw: 0.0,
            pitch: 0.0,
            fov_y: 60.0_f32.to_radians(),
            min_distance: 0.5,
            max_distance: 50.0,
            max_pitch: 89.0_f32.to_radians(),
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        const SENS: f32 = 0.005;
        self.yaw -= dx * SENS;
        self.pitch += dy * SENS;
        self.pitch = self.pitch.clamp(-self.max_pitch, self.max_pitch);
    }

    pub fn zoom(&mut self, dy: f32) {
        const SENS: f32 = 0.01;
        self.distance *= 1.0 + dy * SENS;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }
}
use nalgebra_glm as glm;

pub struct Camera {
    target: glm::Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
    fov_y: f32,

    min_distance: f32,
    max_distance: f32,
    max_pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: glm::vec3(0.0, 0.0, 0.0),
            distance: 3.0,
            yaw: 0.0,
            pitch: 0.0,
            fov_y: 60.0_f32.to_radians(),
            min_distance: 0.5,
            max_distance: 50.0,
            max_pitch: 89.0_f32.to_radians(),
        }
    }

    pub fn position(&self) -> glm::Vec3 {
        let cp = self.pitch.cos();
        let offset = glm::vec3(
            self.distance * cp * self.yaw.sin(),
            self.distance * self.pitch.sin(),
            self.distance * cp * self.yaw.cos(),
        );
        self.target + offset
    }

    pub fn basis(&self) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        let pos = self.position();
        let forward = glm::normalize(&(self.target - pos));
        let world_up = glm::vec3(0.0, 1.0, 0.0);
        let right = glm::normalize(&glm::cross(&forward, &world_up));
        let up = glm::cross(&right, &forward); // already unit-length (both inputs unit & orthogonal)
        (forward, right, up)
    }

    pub fn tan_half_fov(&self) -> f32 {
        (self.fov_y * 0.5).tan()
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
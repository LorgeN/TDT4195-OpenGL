// Constants for making movement a bit more "comfortable"
const MOVEMENT_SPEED: f32 = 100.0;
const SENSITIVITY: f32 = 0.001;

// Allow looking straight up and down
const PITCH_MAX: f32 = std::f32::consts::PI / 2.0;
const YAW_MAX: f32 = std::f32::consts::PI;

#[derive(Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    /// Makes a view projection matrix for the current camera position
    pub fn make_view_transform(&self, fovy: f32) -> glm::Mat4 {
        let mut transformation: glm::Mat4 = glm::Mat4::identity();
        transformation =
            glm::translation(&glm::vec3(-self.x, -self.y, -self.z - 2.0)) * transformation;
        transformation = glm::rotation(self.yaw, &glm::vec3(0.0, 1.0, 0.0)) * transformation;
        transformation = glm::rotation(self.pitch, &glm::vec3(1.0, 0.0, 0.0)) * transformation;
        transformation = glm::perspective(fovy, 45f32, 1.0, 1000.0) * transformation;
        transformation
    }

    /// Updates yaw and pitch based on mouse delta
    pub fn move_mouse(&mut self, x: f32, y: f32) {
        self.yaw = self.yaw + x * SENSITIVITY;
        if self.yaw > YAW_MAX {
            self.yaw -= 2.0 * YAW_MAX;
        } else if self.yaw < -YAW_MAX {
            self.yaw += 2.0 * YAW_MAX;
        }

        self.pitch = self.pitch + y * SENSITIVITY;
        if self.pitch > PITCH_MAX {
            self.pitch = PITCH_MAX;
        } else if self.pitch < -PITCH_MAX {
            self.pitch = -PITCH_MAX;
        }
    }

    // These are pretty self explanatory

    pub fn move_forward(&mut self, delta_time: f32) {
        self.x += MOVEMENT_SPEED * delta_time * self.yaw.sin();
        self.z -= MOVEMENT_SPEED * delta_time * self.yaw.cos();
    }

    pub fn move_backward(&mut self, delta_time: f32) {
        self.x -= MOVEMENT_SPEED * delta_time * self.yaw.sin();
        self.z += MOVEMENT_SPEED * delta_time * self.yaw.cos();
    }

    pub fn move_left(&mut self, delta_time: f32) {
        self.x -= MOVEMENT_SPEED * delta_time * self.yaw.cos();
        self.z -= MOVEMENT_SPEED * delta_time * self.yaw.sin();
    }

    pub fn move_right(&mut self, delta_time: f32) {
        self.x += MOVEMENT_SPEED * delta_time * self.yaw.cos();
        self.z += MOVEMENT_SPEED * delta_time * self.yaw.sin();
    }

    pub fn move_up(&mut self, delta_time: f32) {
        self.y += MOVEMENT_SPEED * delta_time;
    }

    pub fn move_down(&mut self, delta_time: f32) {
        self.y -= MOVEMENT_SPEED * delta_time;
    }
}

// Constants for making movement a bit more "comfortable"
const MOVEMENT_SPEED: f32 = 1.0;
const SENSITIVITY: f32 = 0.001;

const PITCH_MAX: f32 = std::f32::consts::PI;
const YAW_MAX: f32 = std::f32::consts::PI / 2.0;


#[derive(Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
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

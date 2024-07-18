use rubycave::glam::Vec3;

pub trait Entity {
    fn move_by(&mut self, motion: Vec3);
    fn update(&mut self, delta: f32);
    fn get_head(&self) -> Vec3;
    fn move_head(&mut self, rot: Vec3);
    fn get_position(&self) -> Vec3;
}

pub struct Player {
    head: Vec3,
    friction: f32,
    motion: Vec3,
    position: Vec3,
}

impl Player {
    pub fn new(head: Vec3) -> Self {
        Self {
            head: head,
            friction: 1.0 - (1.0 / 64.0),
            motion: Vec3::ZERO,
            position: Vec3::ZERO,
        }
    }
}

impl Entity for Player {
    fn move_by(&mut self, motion: Vec3) {
        self.motion += motion;
    }

    fn update(&mut self, delta: f32) {
        self.motion *= self.friction;
        self.position += self.motion * delta;
    }

    fn get_head(&self) -> Vec3 {
        self.head
    }

    fn move_head(&mut self, rot: Vec3) {
        self.head += rot;
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }
}

use rubycave::glam::Vec3;

pub trait Entity {
    fn get_name(&self) -> &str;
    fn get_position(&self) -> Vec3;
    fn teleport(&mut self, pos: Vec3);
    fn move_by(&mut self, motion: Vec3);
    fn get_head(&self) -> Vec3;
    fn set_head(&mut self, head: Vec3);
    fn move_head(&mut self, rot: Vec3);
    fn update(&mut self, delta: f32);
}

pub struct Player {
    username: String,
    head: Vec3,
    friction: f32,
    motion: Vec3,
    position: Vec3,
}

impl Player {
    pub fn new(username: &str, head: Vec3) -> Self {
        Self {
            username: username.to_owned(),
            head,
            friction: 1.0 - (1.0 / 64.0),
            motion: Vec3::ZERO,
            position: Vec3::ZERO,
        }
    }
}

impl Entity for Player {
    fn get_name(&self) -> &str {
        &self.username
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn teleport(&mut self, pos: Vec3) {
        self.position = pos
    }

    fn move_by(&mut self, motion: Vec3) {
        self.motion += motion;
    }

    fn get_head(&self) -> Vec3 {
        self.head
    }

    fn set_head(&mut self, head: Vec3) {
        self.head = head;
        self.clamp_head();
    }

    fn move_head(&mut self, rot: Vec3) {
        self.head += rot;
        self.clamp_head();
    }

    fn update(&mut self, delta: f32) {
        self.motion *= self.friction;
        self.position += self.motion * delta;
    }
}

impl Player {
    fn clamp_head(&mut self) {
        self.head.y = self
            .head
            .y
            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

        let f2pi = std::f32::consts::PI * 2.0;

        if self.head.x < -f2pi {
            self.head.x += f2pi;
        }

        if self.head.x > f2pi {
            self.head.x -= f2pi;
        }
    }
}

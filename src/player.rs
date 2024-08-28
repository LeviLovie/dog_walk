use super::utils::Point;

pub struct Player {
    pos: Point,
    view: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, view: f32) -> Self {
        Self {
            pos: Point::new(x, y),
            view,
        }
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn view(&self) -> f32 {
        self.view
    }

    pub fn move_forward(&mut self, speed: f32) {
        self.pos.x += self.view.to_radians().cos() * speed;
        self.pos.y += self.view.to_radians().sin() * speed;
    }

    pub fn move_backward(&mut self, speed: f32) {
        self.pos.x -= self.view.to_radians().cos() * speed;
        self.pos.y -= self.view.to_radians().sin() * speed;
    }

    pub fn turn(&mut self, angle: f32) {
        self.view += angle;
        self.view = self.view.rem_euclid(360.0);
    }
}

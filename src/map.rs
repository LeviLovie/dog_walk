use super::utils::{Point, WallType};

pub struct Wall {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub wall_type: WallType,
}

impl Wall {
    pub fn new(x: f32, y: f32, width: f32, height: f32, wall_type: WallType) -> Self {
        Self {
            x,
            y,
            width,
            height,
            wall_type,
        }
    }

    pub fn intersect_with_ray(&self, ray_origin: Point, ray_dir: Point) -> Option<f32> {
        let inv_dir = Point {
            x: 1.0 / ray_dir.x,
            y: 1.0 / ray_dir.y,
        };

        let t1 = (self.x + self.width - ray_origin.x) * inv_dir.x;
        let t2 = (self.x - ray_origin.x) * inv_dir.x;
        let t3 = (self.y + self.height - ray_origin.y) * inv_dir.y;
        let t4 = (self.y - ray_origin.y) * inv_dir.y;

        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));

        if tmax >= tmin && tmin >= 0.0 {
            Some(tmin)
        } else {
            None
        }
    }
}

pub struct Map {
    walls: Vec<Wall>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            walls: vec![
                Wall::new(1.0, 0.0, 19.0, 1.0, WallType::Full),
                Wall::new(0.0, 0.0, 1.0, 9.0, WallType::Full),
                Wall::new(0.0, 9.0, 19.0, 1.0, WallType::Full),
                Wall::new(19.0, 1.0, 1.0, 9.0, WallType::Full),
                Wall::new(15.0, 3.0, 2.0, 4.0, WallType::Full),
                Wall::new(5.0, 3.0, 2.0, 3.0, WallType::Full),
                Wall::new(7.0, 3.0, 3.0, 2.0, WallType::Half),
            ],
        }
    }

    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }
}

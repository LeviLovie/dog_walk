use super::map::Wall;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub enum WallType {
    Full,
    Half,
}

#[derive(Debug)]
pub struct Ray {
    pub dir: Point,
    pub pos: Point,
    pub collisions: Vec<(Point, WallType)>,
}

impl Ray {
    pub fn new(dir: Point, pos: Point) -> Self {
        Self {
            dir,
            pos,
            collisions: Vec::new(),
        }
    }

    // pub fn calculate_intersections(&mut self, walls: &Vec<Wall>) -> Vec<(Point, WallType)> {
    pub fn calculate_intersections(&mut self, walls: &Vec<Wall>) {
        let mut collitions: Vec<(Point, WallType)> = Vec::new();
        for wall in walls {
            if let Some(t) = wall.intersect_with_ray(self.pos, self.dir) {
                let collision_point =
                    Point::new(self.pos.x + self.dir.x * t, self.pos.y + self.dir.y * t);
                collitions.push((collision_point, wall.wall_type.clone()));
            }
        }

        collitions.sort_by(|a, b| {
            let a_dist = ((a.0.x - self.pos.x).powi(2) + (a.0.y - self.pos.y).powi(2)).sqrt();
            let b_dist = ((b.0.x - self.pos.x).powi(2) + (b.0.y - self.pos.y).powi(2)).sqrt();
            a_dist.partial_cmp(&b_dist).unwrap()
        });
        collitions.reverse();

        self.collisions = collitions;
    }

    pub fn len(&self) -> f32 {
        if self.collisions.is_empty() {
            f32::INFINITY
        } else {
            let collision = self.collisions[self.collisions.len() - 1].clone();
            ((collision.0.x - self.pos.x).powi(2) + (collision.0.y - self.pos.y).powi(2)).sqrt()
        }
    }
}

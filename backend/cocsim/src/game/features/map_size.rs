use nalgebra::Vector2;

pub struct MapSize {
    pub base_size: i32,
    pub border_size: i32,
}

impl MapSize {
    pub fn total_size(&self) -> i32 {
        self.base_size + 2 * self.border_size
    }

    pub fn is_inside_map(&self, position: Vector2<i32>) -> bool {
        0 <= position.x
            && position.x < self.total_size()
            && 0 <= position.y
            && position.y < self.total_size()
    }

    pub fn is_border(&self, position: Vector2<i32>) -> bool {
        position.y < self.border_size
            || position.x < self.border_size
            || position.y >= self.base_size + self.border_size
            || position.x >= self.base_size + self.border_size
    }
}

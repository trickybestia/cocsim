use shipyard::Unique;

#[derive(Unique)]
pub struct MapSize {
    pub base_size: i32,
    pub border_size: i32,
}

impl MapSize {
    pub fn total_size(&self) -> i32 {
        self.base_size + 2 * self.border_size
    }

    pub fn is_inside_map(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.total_size() && 0 <= y && y < self.total_size()
    }

    pub fn is_border(&self, x: i32, y: i32) -> bool {
        y < self.border_size
            || x < self.border_size
            || y >= self.base_size + self.border_size
            || x >= self.base_size + self.border_size
    }
}

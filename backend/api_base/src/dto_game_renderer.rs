use cocsim::{
    Game,
    Shape,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub time_elapsed: f32,
    pub progress_info: String,
    pub total_base_size: usize,
    /// None if not changed
    pub grid: Option<Vec<Shape>>,
    /// None if not changed
    pub collision: Option<Vec<Shape>>,
    pub entities: Vec<Shape>,
}

pub struct DtoGameRenderer {
    result: Vec<Frame>,
    ticks_per_draw: usize,
    ticks_since_last_draw: Option<usize>,
}

impl DtoGameRenderer {
    pub fn new(ticks_per_draw: usize) -> Self {
        Self {
            result: Vec::new(),
            ticks_per_draw,
            ticks_since_last_draw: None,
        }
    }

    pub fn draw(&mut self, game: &mut Game) {
        if self.ticks_since_last_draw.is_none()
            || self.ticks_since_last_draw == Some(self.ticks_per_draw)
        {
            self.ticks_since_last_draw = Some(0);

            self.draw_internal(game);
        }

        *self.ticks_since_last_draw.as_mut().unwrap() += 1;
    }

    pub fn finish(mut self, game: &mut Game) -> Vec<Frame> {
        if self.result.last().unwrap().time_elapsed != game.time_elapsed() {
            self.draw_internal(game);
        }

        self.result
    }

    fn draw_internal(&mut self, game: &mut Game) {
        let grid = if self.result.is_empty() {
            Some(game.draw_grid())
        } else {
            None
        };
        let collision = if game.need_redraw_collision() {
            Some(game.draw_collision())
        } else {
            None
        };

        self.result.push(Frame {
            time_elapsed: game.time_elapsed(),
            progress_info: game.progress_info(),
            total_base_size: game.map_size().total_size() as usize,
            grid,
            collision,
            entities: game.draw_entities(),
        });
    }
}

use crate::{
    Game,
    attack_optimizer::AttackPlanUnit,
};

pub struct AttackPlanExecutor {
    units: Vec<AttackPlanUnit>,
}

impl AttackPlanExecutor {
    pub fn new(units: &[AttackPlanUnit]) -> Self {
        let mut units = units.to_owned();

        // sort reversed by drop_time key
        units.sort_unstable_by(|a, b| b.drop_time().total_cmp(&a.drop_time()));

        Self { units }
    }

    pub fn tick(&mut self, game: &mut Game) {
        while !self.units.is_empty()
            && self.units.last().unwrap().drop_time() <= game.time_elapsed()
        {
            let unit = self.units.pop().unwrap();
            let position = unit.cartesian_position(&game.map_size(), &game.drop_zone().0);

            game.spawn_unit(unit.unit_model(), position);
        }
    }
}

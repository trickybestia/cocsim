use nalgebra::Vector2;

use crate::{
    Game,
    UnitModelEnum,
    attack_optimizer::AttackPlanUnitGroup,
    consts::{
        UNIT_DROP_COOLDOWN,
        UNIT_DROP_GROUP_COOLDOWN,
    },
};

struct AttackPlanExecutorUnit {
    pub unit: UnitModelEnum,
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

enum AttackPlanExecutorState {
    Created { groups: Vec<AttackPlanUnitGroup> },
    // Can't access DropZone and MapSize in AttackPlanExecutor::new, moving initialization to
    // first AttackPlanExecutor::tick call
    Initialized { units: Vec<AttackPlanExecutorUnit> },
}

pub struct AttackPlanExecutor {
    state: AttackPlanExecutorState,
}

impl AttackPlanExecutor {
    pub fn new(groups: Vec<AttackPlanUnitGroup>) -> Self {
        Self {
            state: AttackPlanExecutorState::Created { groups },
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.state {
            AttackPlanExecutorState::Created { groups } => groups.is_empty(),
            AttackPlanExecutorState::Initialized { units } => units.is_empty(),
        }
    }

    pub fn tick(&mut self, game: &mut Game) {
        if let AttackPlanExecutorState::Created { groups } = &mut self.state {
            // sort reversed by drop_time key
            groups.sort_unstable_by(|a, b| a.drop_time.total_cmp(&b.drop_time));

            let mut units = Vec::new();
            let mut next_drop_time = 0.0f32;

            for group in groups {
                let position = group.cartesian_position(&game.map_size, &game.drop_zone.0);

                next_drop_time = next_drop_time.max(group.drop_time);

                for _ in 0..group.count {
                    units.push(AttackPlanExecutorUnit {
                        unit: group.unit_model.clone(),
                        position,
                        drop_time: next_drop_time,
                    });

                    next_drop_time += UNIT_DROP_COOLDOWN;
                }

                next_drop_time += UNIT_DROP_GROUP_COOLDOWN;
            }

            units.reverse();

            self.state = AttackPlanExecutorState::Initialized { units }
        }

        if let AttackPlanExecutorState::Initialized { units } = &mut self.state {
            while !units.is_empty() && units.last().unwrap().drop_time <= game.time_elapsed() {
                let unit = units.pop().unwrap();

                game.spawn_attack_unit(&unit.unit, unit.position);
            }
        } else {
            unreachable!();
        }
    }
}

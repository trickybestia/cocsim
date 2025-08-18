use hecs::PreparedQuery;

use crate::{
    Game,
    game::features::{
        actions::Action,
        to_be_despawned::ToBeDespawned,
    },
    utils::AnyMapExt,
};

pub struct TickSpell {
    pub remaining_ticks: usize,
    pub time_per_tick: f32,
    pub remaining_time_to_next_tick: f32,
    pub tick: Box<dyn Action>,
}

pub fn update(game: &mut Game) {
    let mut ticks = Vec::new();
    let mut to_be_despawned = Vec::new();

    for (id, spell) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&mut TickSpell>>()
        .query_mut(&mut game.world)
    {
        spell.remaining_time_to_next_tick -= game.delta_time;

        if spell.remaining_time_to_next_tick <= 0.0 {
            ticks.push((id, spell.tick.clone()));

            spell.remaining_ticks -= 1;

            spell.remaining_time_to_next_tick = spell.time_per_tick;

            if spell.remaining_ticks == 1 {
                to_be_despawned.push(id);
            }
        }
    }

    for (id, tick) in ticks {
        tick.call(id, game);
    }

    for id in to_be_despawned {
        game.world.insert_one(id, ToBeDespawned).unwrap();
    }
}

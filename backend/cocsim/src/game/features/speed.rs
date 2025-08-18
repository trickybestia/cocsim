use hecs::PreparedQuery;

use crate::{
    Game,
    utils::AnyMapExt,
};

pub struct Speed {
    /// Raw speed, before applying any modifiers (like spells).
    pub raw: f32,
    /// Speed after applying modifiers (like spells).
    pub real: f32,
}

pub struct HasteSpellSpeedModifier {
    pub amount: f32,
    pub remaining_time: f32,
}

impl HasteSpellSpeedModifier {
    pub fn update(game: &mut Game) {
        let mut remove_modifier_ids = Vec::new();

        for (id, (speed, modifier)) in game
            .cache
            .get_mut_or_default::<PreparedQuery<(&mut Speed, &mut HasteSpellSpeedModifier)>>()
            .query_mut(&mut game.world)
        {
            speed.real += modifier.amount;

            modifier.remaining_time -= game.delta_time;

            if modifier.remaining_time <= 0.0 {
                remove_modifier_ids.push(id);
            }
        }

        for id in remove_modifier_ids {
            game.world
                .remove_one::<HasteSpellSpeedModifier>(id)
                .unwrap();
        }
    }
}

pub fn reset_modifiers(game: &mut Game) {
    for (_, speed) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&mut Speed>>()
        .query_mut(&mut game.world)
    {
        speed.real = speed.raw;
    }
}

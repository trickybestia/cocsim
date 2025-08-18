use hecs::PreparedQuery;

use crate::{
    Game,
    utils::AnyMapExt,
};

pub struct DamageMultiplier {
    /// Default value is 1.0.
    pub value: f32,
}

macro_rules! declare_damage_modififer {
    ($name:ident) => {
        pub struct $name {
            /// Value to be added to [`DamageMultiplier::value`].
            pub amount: f32,
            pub remaining_time: f32,
        }

        impl $name {
            pub fn update(game: &mut Game) {
                let mut remove_modifier_ids = Vec::new();

                for (id, (multiplier, modifier)) in game
                    .cache
                    .get_mut_or_default::<PreparedQuery<(&mut DamageMultiplier, &mut Self)>>()
                    .query_mut(&mut game.world)
                {
                    multiplier.value += modifier.amount;

                    modifier.remaining_time -= game.delta_time;

                    if modifier.remaining_time <= 0.0 {
                        remove_modifier_ids.push(id);
                    }
                }

                for id in remove_modifier_ids {
                    game.world.remove_one::<Self>(id).unwrap();
                }
            }
        }
    };
}
declare_damage_modififer!(RageSpellDamageModifier);

pub fn reset_modifiers(game: &mut Game) {
    for (_, multiplier) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&mut DamageMultiplier>>()
        .query_mut(&mut game.world)
    {
        multiplier.value = 1.0;
    }
}

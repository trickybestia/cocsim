use hecs::PreparedQuery;
use nalgebra::Vector2;

use crate::{
    Game,
    colliders::Collider,
    consts::UNIT_DISTANCE_TO_WAYPOINT_EPS,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        position::Position,
    },
    units::{
        UnitModel,
        UnitModelEnum,
    },
    utils::AnyMapExt,
};

pub struct ClanCastle {
    /// Remaining (not deployed yet) units sorted by
    /// [UnitModelEnum::clan_castle_deployment_cmp] reversed.
    pub units: Vec<UnitModelEnum>,
    pub unit_deploy_cooldown: f32,
    /// When unit is deployed, this should be set to [`Game::time_elapsed`] +
    /// [`Self::unit_deploy_cooldown`]
    pub remaining_unit_deploy_cooldown: f32,
    pub unit_deploy_trigger_range: f32,
}

#[derive(Default)]
struct ClanCastleUpdateCache<'a> {
    pub clan_castle_query: PreparedQuery<(&'a mut ClanCastle, &'a Position, &'a Team)>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a Position, &'a Team)>,
}

pub fn update(game: &mut Game) {
    let cache = game.cache.get_mut_or_default::<ClanCastleUpdateCache>();
    let mut units_to_spawn: Vec<(UnitModelEnum, Vector2<f32>, Team)> = Vec::new();

    for (_clan_castle_id, (clan_castle, clan_castle_position, clan_castle_team)) in
        cache.clan_castle_query.query(&game.world).iter()
    {
        if clan_castle.units.is_empty() {
            continue;
        }

        clan_castle.remaining_unit_deploy_cooldown -= game.delta_time;

        if clan_castle.remaining_unit_deploy_cooldown > 0.0 {
            continue;
        }

        let mut has_air_target = false;
        let mut has_ground_target = false;

        for (_target_id, (attack_target, target_position, target_team)) in
            cache.target_query.query(&game.world).iter()
        {
            if target_team == clan_castle_team {
                continue;
            }

            let target_in_trigger_range = attack_target
                .collider
                .translate(target_position.0)
                .attack_area(clan_castle.unit_deploy_trigger_range + UNIT_DISTANCE_TO_WAYPOINT_EPS)
                .contains(clan_castle_position.0);

            if !target_in_trigger_range {
                continue;
            }

            if attack_target.flags.contains(AttackTargetFlags::AIR) {
                has_air_target = true;
            }

            if attack_target.flags.contains(AttackTargetFlags::GROUND) {
                has_ground_target = true;
            }

            if has_air_target && has_ground_target {
                // no need to check for targets further
                break;
            }
        }

        for i in (0..clan_castle.units.len()).rev() {
            if has_air_target && clan_castle.units[i].r#type().attack_air
                || has_ground_target && clan_castle.units[i].r#type().attack_ground
            {
                units_to_spawn.push((
                    clan_castle.units.remove(i),
                    clan_castle_position.0,
                    *clan_castle_team,
                ));

                clan_castle.remaining_unit_deploy_cooldown = clan_castle.unit_deploy_cooldown;

                break;
            }
        }
    }

    for (unit_model, unit_position, unit_team) in units_to_spawn {
        unit_model.spawn(&mut game.world, unit_position, unit_team);
    }
}

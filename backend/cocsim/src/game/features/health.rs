use nalgebra::Vector2;
use shipyard::{
    AddComponent,
    AllStoragesViewMut,
    Component,
    EntitiesViewMut,
    EntityId,
    IntoIter,
    View,
    ViewMut,
    sparse_set::SparseSet,
};

use crate::{
    colliders::Collider,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        position::Position,
        to_be_deleted::ToBeDeleted,
    },
};

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct EntityDamageEvent {
    pub target: EntityId,
    pub damage: f32,
}

#[derive(Component)]
pub struct SplashDamageEvent {
    pub attacker_team: Team,
    pub damage_ground: bool,
    pub damage_air: bool,
    pub target: Vector2<f32>,
    pub damage: f32,
    pub radius: f32,
}

pub fn handle_entity_damage_events(
    mut v_health: ViewMut<Health>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
    v_entity_damage_event: View<EntityDamageEvent>,
) {
    for damage_event in v_entity_damage_event.iter() {
        let target_health = &mut v_health[damage_event.target];

        target_health.0 -= damage_event.damage;
    }

    for (id, health) in v_health.iter().with_id() {
        if health.0 <= 0.0 {
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}

pub fn handle_splash_damage_events(
    mut entities: EntitiesViewMut,
    mut v_entity_damage_event: ViewMut<EntityDamageEvent>,
    v_splash_damage_event: View<SplashDamageEvent>,
    v_position: View<Position>,
    v_team: View<Team>,
    v_attack_target: View<AttackTarget>,
) {
    for splash_damage_event in v_splash_damage_event.iter() {
        for (id, (position, team, attack_target)) in
            (&v_position, &v_team, &v_attack_target).iter().with_id()
        {
            if splash_damage_event.attacker_team == *team {
                continue;
            }

            if !((splash_damage_event.damage_air
                && attack_target.flags.contains(AttackTargetFlags::AIR))
                || (splash_damage_event.damage_ground
                    && attack_target.flags.contains(AttackTargetFlags::GROUND)))
            {
                continue;
            }

            if attack_target
                .collider
                .translate(position.0)
                .attack_area(splash_damage_event.radius)
                .contains(splash_damage_event.target)
            {
                entities.add_entity(
                    &mut v_entity_damage_event,
                    EntityDamageEvent {
                        target: id,
                        damage: splash_damage_event.damage,
                    },
                );
            }
        }
    }
}

pub fn cleanup_events(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<EntityDamageEvent>>();
    all_storages.delete_any::<SparseSet<SplashDamageEvent>>();
}

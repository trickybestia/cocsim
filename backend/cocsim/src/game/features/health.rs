use shipyard::{
    AddComponent,
    AllStoragesViewMut,
    Component,
    EntityId,
    IntoIter,
    View,
    ViewMut,
    sparse_set::SparseSet,
};

use crate::game::features::to_be_deleted::ToBeDeleted;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct DamageEvent {
    pub target: EntityId,
    pub damage: f32,
}

pub fn handle_damage_events(
    mut v_health: ViewMut<Health>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
    v_damage_event: View<DamageEvent>,
) {
    for damage_event in v_damage_event.iter() {
        let target_health = &mut v_health[damage_event.target];

        target_health.0 -= damage_event.damage;
    }

    for (id, health) in v_health.iter().with_id() {
        if health.0 <= 0.0 {
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}

pub fn cleanup_events(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<DamageEvent>>();
}

use shipyard::{
    AddComponent,
    AllStoragesViewMut,
    Component,
    IntoIter,
    View,
    ViewMut,
    sparse_set::SparseSet,
};

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct DamageRequest(pub f32);

#[derive(Component)]
pub struct DeathRequest;

pub fn handle_damage_requests(
    mut v_health: ViewMut<Health>,
    v_damage_request: View<DamageRequest>,
    mut v_death_request: ViewMut<DeathRequest>,
) {
    for (health, damage_request) in (&mut v_health, &v_damage_request).iter() {
        health.0 = health.0 - damage_request.0;
    }

    for (id, health) in v_health.iter().with_id() {
        if health.0 <= 0.0 {
            v_death_request.add_component_unchecked(id, DeathRequest);
        }
    }
}

pub fn handle_death_requests(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<DeathRequest>>();
}

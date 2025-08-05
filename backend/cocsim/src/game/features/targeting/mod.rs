pub mod air_unit;
pub mod building;

use shipyard::{
    AllStoragesViewMut,
    Component,
    ViewMut,
};

#[derive(Component)]
pub struct FindTargetRequest;

pub fn handle_find_target_requests(all_storages: AllStoragesViewMut) {
    all_storages.run(air_unit::handle_find_target_requests);
    all_storages.run(building::handle_find_target_requests);

    all_storages
        .borrow::<ViewMut<FindTargetRequest>>()
        .unwrap()
        .clear();
}

use shipyard::{
    AllStoragesViewMut,
    Component,
    sparse_set::SparseSet,
};

#[derive(Component)]
pub struct Event;

pub fn cleanup_events(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<Event>>();
}

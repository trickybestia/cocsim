use shipyard::{
    AllStoragesViewMut,
    Component,
    sparse_set::SparseSet,
};

/// To be used only for event entities which should be removed after event
/// handling.
#[derive(Component)]
pub struct Event;

pub fn cleanup_events(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<Event>>();
}

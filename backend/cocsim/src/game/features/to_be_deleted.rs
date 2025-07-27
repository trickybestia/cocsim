use shipyard::{
    AllStoragesViewMut,
    Component,
    sparse_set::SparseSet,
};

#[derive(Component)]
pub struct ToBeDeleted;

pub fn handle_to_be_deleted(mut all_storages: AllStoragesViewMut) {
    all_storages.delete_any::<SparseSet<ToBeDeleted>>();
}

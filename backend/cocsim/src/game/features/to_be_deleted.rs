use shipyard::{
    AllStoragesViewMut,
    Component,
    EntityId,
    IntoIter,
    View,
    sparse_set::SparseSet,
};

use crate::game::features::actions::{
    Action,
    ActionEnum,
};

#[derive(Component)]
pub struct ToBeDeleted;

#[derive(Component)]
pub struct OnDelete(pub ActionEnum);

pub fn handle_to_be_deleted(mut all_storages: AllStoragesViewMut) {
    let on_deleted_queue = all_storages.run(create_on_delete_queue);

    for (id, action) in on_deleted_queue {
        action.call(id, &mut all_storages);
    }

    all_storages.delete_any::<SparseSet<ToBeDeleted>>();
}

fn create_on_delete_queue(
    v_to_be_deleted: View<ToBeDeleted>,
    v_on_delete: View<OnDelete>,
) -> Vec<(EntityId, ActionEnum)> {
    let mut result = Vec::new();

    for (id, (_, on_delete)) in (&v_to_be_deleted, &v_on_delete).iter().with_id() {
        result.push((id, on_delete.0.clone()));
    }

    result
}

use shipyard::{
    AddComponent,
    AllStoragesViewMut,
    Component,
    IntoIter,
    View,
    ViewMut,
};

use crate::game::features::collision::{
    ColliderComponent,
    UpdateCollisionRequest,
};

#[derive(Component)]
pub struct Created;

pub fn handle_created(all_storages: AllStoragesViewMut) {
    all_storages.run(request_update_collision_on_created_colliders);

    all_storages.borrow::<ViewMut<Created>>().unwrap().clear();
}

fn request_update_collision_on_created_colliders(
    mut v_update_collision_request: ViewMut<UpdateCollisionRequest>,
    v_created: View<Created>,
    v_collider: View<ColliderComponent>,
) {
    for (id, _) in (&v_created, &v_collider).iter().with_id() {
        v_update_collision_request.add_component_unchecked(id, UpdateCollisionRequest);
    }
}

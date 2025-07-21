use shipyard::{
    AllStoragesView,
    Component,
    EntityId,
    IntoIter,
    View,
};

use crate::Shape;

#[derive(Component)]
struct Drawable {
    pub draw_fn: fn(EntityId, &AllStoragesView, &mut Vec<Shape>),
}

pub fn draw(shapes: &mut Vec<Shape>, all_storages: AllStoragesView) {
    let v_drawable = all_storages.borrow::<View<Drawable>>().unwrap();

    for (id, drawable) in v_drawable.iter().with_id() {
        (drawable.draw_fn)(id, &all_storages, shapes);
    }
}

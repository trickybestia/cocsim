mod collider;
mod list_collider;
mod point_collider;
mod rect_collider;

pub use collider::Collider;
use enum_dispatch::enum_dispatch;
pub use list_collider::ListCollider;
use nalgebra::Vector2;
pub use point_collider::PointCollider;
pub use rect_collider::RectCollider;

#[enum_dispatch(Collider)]
pub enum ColliderEnum {
    ListCollider,
    RectCollider,
    PointCollider,
}

use crate::components::base_components::BaseEntityComponents;
use amethyst::core::Transform;

pub fn update_transform(base_components: &BaseEntityComponents, transform: &mut Transform) {
    transform.set_translation_x(base_components.position.x);
    transform.set_translation_y(base_components.position.y);
}

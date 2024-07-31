use bevy::prelude::*;

pub fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|entity_id| {
        commands.entity(entity_id).despawn_recursive();
    });
}

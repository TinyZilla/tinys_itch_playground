use bevy::prelude::*;

use crate::spaceship::Spaceship;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_transform);
    }
}

fn print_transform(query: Query<(Entity, &Transform), With<Spaceship>>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} has transform {:?}", entity, transform);
    }
}

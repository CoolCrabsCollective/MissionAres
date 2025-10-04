use bevy::{ecs::resource::Resource, math::IVec2};

#[derive(Resource)]
pub struct Rover {
    pub position: IVec2,
}

pub struct RoverPlugin;
